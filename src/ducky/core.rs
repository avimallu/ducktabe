use arrow::datatypes::DataType;
use comfy_table::presets::{NOTHING, UTF8_FULL_CONDENSED};
use comfy_table::Table as CTable;
use duckdb::arrow::util::display::{ArrayFormatter, FormatOptions};
use duckdb::{arrow::array::RecordBatch, Connection};
use std::collections::HashMap;
use std::{path::PathBuf, time::SystemTime};
use thiserror;

use crate::ducky::utils;

#[derive(Debug)]
pub struct DuckDBInMemoryConnection {
    conn: Connection,
}

#[derive(Debug, thiserror::Error)]
pub enum DuckyError {
    #[error("DuckDB error:\n{0}")]
    DuckDB(#[from] duckdb::Error),
    #[error("IO error:\n{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Generic(String),
}

impl DuckDBInMemoryConnection {
    pub fn new() -> DuckDBInMemoryConnection {
        let conn = Connection::open_in_memory().unwrap();
        return DuckDBInMemoryConnection { conn };
    }

    pub fn create_view_from_path(
        &self,
        view_name: &str,
        path_name: &str,
    ) -> Result<(), DuckyError> {
        // Allow the user to use SELECT * FROM df for ease of use
        let root_statement = format!(
            "CREATE OR REPLACE VIEW {} AS SELECT * FROM '{}'",
            view_name, path_name
        );
        let mut stmt = self.conn.prepare(&root_statement)?;
        stmt.execute([])?;
        Ok(())
    }

    pub fn get_path_schema(
        &self,
        path_name: &str,
    ) -> Result<(Vec<String>, Vec<String>), DuckyError> {
        let root_statement = format!("SELECT * FROM '{}'", path_name);
        let mut stmt = self.conn.prepare(&root_statement)?;
        let _ = stmt.execute([])?;
        let columns = stmt.column_names();
        let column_types: Vec<String> = columns
            .iter()
            .map(|x| stmt.column_type(stmt.column_index(x).unwrap()).to_string())
            .collect();
        return Ok((columns, column_types));
    }

    pub fn execute_query(&self, query: &str) -> Result<Vec<RecordBatch>, DuckyError> {
        let limited_query = format!("SELECT * FROM ({}) LIMIT 2048", query);
        let mut stmt = self.conn.prepare(&limited_query)?;
        let results = stmt.query_arrow([])?;
        Ok(results.take(5).collect())
    }
}

struct ArrowRecordBatchUtils {
    batches: Vec<RecordBatch>,
}

impl ArrowRecordBatchUtils {
    pub fn batch_to_stringified_rows(
        &self,
        batch_index: usize,
    ) -> Result<Vec<Vec<String>>, DuckyError> {
        let batch = self
            .batches
            .get(batch_index)
            .ok_or(DuckyError::Generic("Out of bounds".to_string()))?;
        let options = FormatOptions::default().with_null("<NULL>");
        let formatters: Vec<ArrayFormatter> = batch
            .columns()
            .iter()
            .map(|x| ArrayFormatter::try_new(x, &options))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DuckyError::Generic(e.to_string()))?;
        let (cols, rows) = (batch.num_columns(), batch.num_rows());

        let mut out = Vec::new();

        for row in 0..rows {
            let mut row_vec = Vec::new();
            for col in 0..cols {
                row_vec.push(formatters[col].value(row).to_string());
            }
            out.push(row_vec)
        }

        Ok(out)
    }

    fn dtype_to_string(&self, x: &DataType) -> String {
        let full = x.to_string();
        let name = full
            .split_once("(")
            .map_or(full.clone(), |(n, _)| n.to_string());
        utils::simplify_arrow_type(&name)
            .map(String::from)
            .unwrap_or(full)
    }

    pub fn batch_schema(
        &self,
        batch_index: usize,
    ) -> Result<(Vec<String>, Vec<String>), DuckyError> {
        let batch = self
            .batches
            .get(batch_index)
            .ok_or(DuckyError::Generic("Out of bounds".to_string()))?;
        let fields = batch.schema().fields.clone();
        let cols = fields.iter().map(|x| x.name().clone()).collect();
        let dtypes = fields
            .iter()
            .map(|x| self.dtype_to_string(x.data_type()))
            .collect();
        Ok((cols, dtypes))
    }
}

#[derive(Debug)]
pub struct Table {
    path: PathBuf,
    // A simple check to re-run table creation
    // when the file has changed in some fashion.
    modified: SystemTime,
    pub columns: Vec<String>,
    pub column_types: Vec<String>,
}

impl Table {
    pub fn get_modified_time(path: &PathBuf) -> SystemTime {
        path.metadata().unwrap().modified().unwrap()
    }

    pub fn new(path: PathBuf, conn: &DuckDBInMemoryConnection) -> Result<Table, DuckyError> {
        let path_as_str = path.to_str().unwrap();

        if !path.exists() {
            return Err(DuckyError::Generic(format!(
                "The provided path '{}' does not exist",
                path_as_str
            )));
        } else if !path.is_file() {
            return Err(DuckyError::Generic(format!(
                "The provided path '{}' is not a file",
                path_as_str
            )));
        }

        let modified = Table::get_modified_time(&path);
        let (columns, column_types) = conn.get_path_schema(path_as_str)?;
        conn.create_view_from_path("df", path_as_str)?;

        Ok(Table {
            path,
            modified,
            columns,
            column_types,
        })
    }

    pub fn query_peek(
        &self,
        query_string: &str,
        conn: &DuckDBInMemoryConnection,
    ) -> Result<CTable, DuckyError> {
        let batches = conn.execute_query(query_string)?;
        if batches.len() == 0 {
            return Ok(CTable::new()
                .load_preset(NOTHING)
                .set_header(vec!["Query returning 0 rows"])
                .clone());
        }
        let arbu = ArrowRecordBatchUtils { batches };
        let (cols, dtypes) = arbu.batch_schema(0)?;
        let rows = arbu.batch_to_stringified_rows(0)?;

        let header: Vec<String> = cols
            .iter()
            .zip(dtypes.iter())
            .map(|(x, y)| {
                let mut col_name = String::new();
                col_name.push_str(x);
                col_name.push_str("\n---\n");
                col_name.push_str(y);
                col_name
            })
            .collect();

        let mut table = CTable::new();
        Ok(table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(header)
            .add_rows(rows)
            .clone())
    }
}
