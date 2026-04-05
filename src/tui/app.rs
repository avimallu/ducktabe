use comfy_table::ContentArrangement::{Disabled, Dynamic};
use comfy_table::Table as CTable;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    style::Style,
    widgets::{Block, Borders},
};
use ratatui_textarea::TextArea;
use std::path::PathBuf;
use unicode_width::UnicodeWidthStr;

use crate::ducky::core::{DuckDBInMemoryConnection, DuckyError, Table};

pub struct App<'a> {
    table: Result<Table, DuckyError>,
    conn: DuckDBInMemoryConnection,
    current_table: CTable,
    current_output_width: u16,
    current_output_height: u16,
    viewport_width: Option<u16>,
    viewport_height: Option<u16>,
    pub query_editor: TextArea<'a>,
    pub current_output: String,
    pub wrap_output: bool,
    pub scroll_coord: (u16, u16),
}

fn error_table(msg: String) -> CTable {
    let mut table = CTable::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.add_row(vec![msg]);
    table
}

impl App<'_> {
    pub fn new(path: PathBuf) -> App<'static> {
        let conn = DuckDBInMemoryConnection::new();
        let table = Table::new(path, &conn);

        let mut text_area = TextArea::new(vec!["SELECT * ".to_string(), "FROM df".to_string()]);
        let textarea_style = Style::default().bg(ratatui::style::Color::DarkGray);
        text_area.set_line_number_style(textarea_style);
        text_area.set_cursor_line_style(Style::default());
        text_area.set_block(Block::default().borders(Borders::ALL));
        let mut app = App {
            table,
            conn,
            query_editor: text_area,
            current_table: CTable::new(),
            current_output: "".to_string(),
            current_output_width: 0,
            current_output_height: 0,
            viewport_width: None,
            viewport_height: None,
            wrap_output: true,
            scroll_coord: (0, 0),
        };
        app.update_query(None);
        app.update_output();
        app
    }

    pub fn get_user_query(&self) -> String {
        self.query_editor.lines().join(" ").replace("\n", "")
    }

    pub fn update_query(&mut self, query: Option<&str>) {
        let query = query.unwrap_or("SELECT * FROM df");
        let current_table = match self.table.as_ref() {
            Ok(table) => match table.query_peek(query, &self.conn) {
                Ok(ctable) => ctable,
                Err(e) => error_table(e.to_string()),
            },
            Err(e) => error_table(e.to_string()),
        };
        self.current_table = current_table;
    }

    pub fn update_output(&mut self) {
        let content_arrangement = if self.wrap_output {
            Dynamic
        } else {
            Disabled
            // DynamicFullWidth
        };
        self.current_table
            .set_content_arrangement(content_arrangement);
        self.current_output = self.current_table.to_string();
        self.current_output_width = self
            .current_output
            .lines()
            .next()
            // Uses UnicodeWidthStr
            .map(|x| x.width() as u16)
            .unwrap_or(0);
        self.current_output_height = self.current_output.lines().count() as u16;
    }

    pub fn output_header_and_content(&self) -> (String, String) {
        let header_loc = self.current_output.lines().position(|l| l.contains("═"));
        if let Some(pos) = header_loc {
            (
                self.current_output
                    .lines()
                    .take(pos + 1)
                    .collect::<Vec<&str>>()
                    .join("\n"),
                self.current_output
                    .lines()
                    .skip(pos + 1)
                    .collect::<Vec<&str>>()
                    .join("\n"),
            )
        } else {
            panic!("Internal error: header location cannot be determined")
        }
    }

    pub fn set_viewport_dims(&mut self, vh: u16, vw: u16) {
        self.viewport_height = Some(vh);
        self.viewport_width = Some(vw);
    }

    pub fn update_scroll(&mut self, key: KeyCode) {
        let max_height = self
            .current_output_height
            .saturating_sub(self.viewport_height.unwrap_or(0));
        let max_width = self
            .current_output_width
            .saturating_sub(self.viewport_width.unwrap_or(0));
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_coord.0 = self.scroll_coord.0.saturating_sub(25).clamp(0, max_height)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_coord.0 = self.scroll_coord.0.saturating_add(25).clamp(0, max_height)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.scroll_coord.1 = self.scroll_coord.1.saturating_sub(25).clamp(0, max_width)
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.scroll_coord.1 = self.scroll_coord.1.saturating_add(25).clamp(0, max_width)
            }
            _ => {}
        }
    }
}
