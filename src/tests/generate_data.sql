-- 
-- ============================================================
-- Setup
-- ============================================================
CREATE TYPE mood AS ENUM ('happy', 'sad', 'neutral', 'excited', 'angry');
CREATE TYPE color AS ENUM ('red', 'green', 'blue', 'yellow', 'purple');

-- ============================================================
-- 1. Integers
-- ============================================================
CREATE OR REPLACE TABLE t1 AS
SELECT
    (i % 120)::TINYINT AS tinyint_col,
    (i * 111)::SMALLINT AS smallint_col,
    (i * 10000)::INTEGER AS int_col,
    (i * 1000000000)::BIGINT AS bigint_col,
    (i * 100000000000000000)::HUGEINT AS hugeint_col,
    (i + 1)::UTINYINT AS utinyint_col,
    (i * 111)::USMALLINT AS usmallint_col,
    (i * 10000)::UINTEGER AS uint_col,
    (i * 1000000000)::UBIGINT AS ubigint_col
FROM generate_series(1, 12) AS t(i);
COPY t1 TO 'test_data/integers.parquet' (FORMAT PARQUET);

-- ============================================================
-- 2. Floats + Decimals
-- ============================================================
CREATE OR REPLACE TABLE t2 AS
SELECT
    (i * 1.5)::FLOAT AS float_col,
    (i * 3.14159265358979)::DOUBLE AS double_col,
    (i * 99.99)::DECIMAL(10,2) AS dec_10_2,
    (i * 12345.678901)::DECIMAL(18,6) AS dec_18_6,
    (i * 0.00001)::DECIMAL(38,10) AS dec_38_10
FROM generate_series(1, 12) AS t(i);
COPY t2 TO 'test_data/floats_decimals.parquet' (FORMAT PARQUET);

-- ============================================================
-- 3. Strings + Enums
-- ============================================================
CREATE OR REPLACE TABLE t3 AS
SELECT
    concat('name_', i)::VARCHAR AS name_col,
    concat('A longer description for item number ', i, '.')::VARCHAR AS desc_col,
    lpad(i::VARCHAR, 6, '0')::VARCHAR AS code_col,
    (CASE i%5 WHEN 0 THEN 'happy' WHEN 1 THEN 'sad' WHEN 2 THEN 'neutral'
              WHEN 3 THEN 'excited' ELSE 'angry' END)::mood AS mood_col,
    (CASE i%5 WHEN 0 THEN 'red' WHEN 1 THEN 'green' WHEN 2 THEN 'blue'
              WHEN 3 THEN 'yellow' ELSE 'purple' END)::color AS color_col,
    CASE WHEN i%3=0 THEN NULL ELSE concat('cat_', i%4) END AS nullable_cat
FROM generate_series(1, 12) AS t(i);
COPY t3 TO 'test_data/strings_enums.parquet' (FORMAT PARQUET);

-- ============================================================
-- 4. Dates + Times
-- ============================================================
CREATE OR REPLACE TABLE t4 AS
SELECT
    ('2024-01-01'::DATE + INTERVAL (i) DAY)::DATE AS date_col,
    ('08:30:00'::TIME + INTERVAL (i) HOUR)::TIME AS time_col,
    ('2024-01-15 10:00:00'::TIMESTAMP + INTERVAL (i*3) HOUR)::TIMESTAMP AS ts_col,
    ('2024-06-01 12:00:00+00'::TIMESTAMPTZ + INTERVAL (i) HOUR)::TIMESTAMPTZ AS tstz_col,
    ('2024-01-01 00:00:00'::TIMESTAMP_S + INTERVAL (i) DAY)::TIMESTAMP_S AS ts_s_col,
    ('2024-01-01 00:00:00'::TIMESTAMP_MS + INTERVAL (i) DAY)::TIMESTAMP_MS AS ts_ms_col,
    ('2024-01-01 00:00:00'::TIMESTAMP_NS + INTERVAL (i) DAY)::TIMESTAMP_NS AS ts_ns_col,
    (INTERVAL (i) DAY + INTERVAL (i*2) HOUR) AS interval_col
FROM generate_series(1, 12) AS t(i);
COPY t4 TO 'test_data/dates.parquet' (FORMAT PARQUET);

-- ============================================================
-- 5. Complex: Struct, List, Nested List, Array, Map
-- ============================================================
CREATE OR REPLACE TABLE t5 AS
SELECT
    {'name': concat('p_', i), 'age': 20+i, 'active': i%2=0} AS struct_col,
    [i, i*2, i*3]::INT[] AS list_col,
    [[i, i+1], [i+2, i+3]]::INT[][] AS nested_list_col,
    [i, i+10, i+20]::INT[3] AS array3_col,
    MAP{concat('a',i): i*10, concat('b',i): i*20} AS map_col,
FROM generate_series(1, 12) AS t(i);
COPY t5 TO 'test_data/complex.parquet' (FORMAT PARQUET);

-- ============================================================
-- 6. Bits, UUID, Blobs, NULLs
-- ============================================================
CREATE OR REPLACE TABLE t6 AS
SELECT
    uuid() AS uuid_col,
    encode(concat('data_', i))::BLOB AS blob_col,
    CASE i%4 WHEN 0 THEN '10100011'::BIT WHEN 1 THEN '11001100'::BIT
             WHEN 2 THEN '00110011'::BIT ELSE '11110000'::BIT END AS bit_col,
    (i%2=0)::BOOLEAN AS bool_col,
    CASE WHEN i%3=0 THEN NULL ELSE i END AS nullable_int,
    CASE WHEN i%4=0 THEN NULL ELSE concat('v_',i) END AS nullable_str
FROM generate_series(1, 12) AS t(i);
COPY t6 TO 'test_data/misc.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 1: integer, double, varchar, date, struct
-- ============================================================
CREATE OR REPLACE TABLE m1 AS
SELECT
    (i * 1000)::INTEGER AS id,
    (i * 2.718)::DOUBLE AS value,
    concat('item_', i)::VARCHAR AS label,
    ('2024-03-01'::DATE + INTERVAL (i) DAY)::DATE AS created,
    {'x': i, 'y': i*2} AS coords
FROM generate_series(1, 12) AS t(i);
COPY m1 TO 'test_data/mixed_1.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 2: bigint, float, enum, timestamp, list, uuid
-- ============================================================
CREATE OR REPLACE TABLE m2 AS
SELECT
    (i * 999999)::BIGINT AS big_id,
    (i * 0.333)::FLOAT AS ratio,
    (CASE i%5 WHEN 0 THEN 'happy' WHEN 1 THEN 'sad' WHEN 2 THEN 'neutral'
              WHEN 3 THEN 'excited' ELSE 'angry' END)::mood AS feeling,
    ('2024-07-01 00:00:00'::TIMESTAMP + INTERVAL (i*7) HOUR)::TIMESTAMP AS event_ts,
    [i, i+5, i+10]::INT[] AS scores,
    uuid() AS row_uuid
FROM generate_series(1, 12) AS t(i);
COPY m2 TO 'test_data/mixed_2.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 3: smallint, decimal, blob, timestamptz, map
-- ============================================================
CREATE OR REPLACE TABLE m3 AS
SELECT
    (i * 50)::SMALLINT AS code,
    (i * 49.95)::DECIMAL(10,2) AS price,
    encode(concat('pk_', i))::BLOB AS raw_data,
    ('2024-09-15 16:00:00+00'::TIMESTAMPTZ + INTERVAL (i*2) HOUR)::TIMESTAMPTZ AS logged_at,
    MAP{concat('attr_',i): i*5} AS attributes
FROM generate_series(1, 12) AS t(i);
COPY m3 TO 'test_data/mixed_3.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 4: utinyint, double, varchar, time, nested list, boolean
-- ============================================================
CREATE OR REPLACE TABLE m4 AS
SELECT
    (i+1)::UTINYINT AS slot,
    (i * 9.81)::DOUBLE AS measurement,
    concat('sensor_', i)::VARCHAR AS sensor_name,
    ('06:00:00'::TIME + INTERVAL (i*30) MINUTE)::TIME AS reading_time,
    [[i, i*2], [i*3, i*4]]::INT[][] AS matrix,
    (i%2=0)::BOOLEAN AS is_valid
FROM generate_series(1, 12) AS t(i);
COPY m4 TO 'test_data/mixed_4.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 5: uinteger, decimal, color enum, interval, nullable
-- ============================================================
CREATE OR REPLACE TABLE m5 AS
SELECT
    (i * 50000)::UINTEGER AS counter,
    (i * 199.99)::DECIMAL(18,6) AS balance,
    (CASE i%5 WHEN 0 THEN 'red' WHEN 1 THEN 'green' WHEN 2 THEN 'blue'
              WHEN 3 THEN 'yellow' ELSE 'purple' END)::color AS tint,
    (INTERVAL (i) HOUR + INTERVAL (i*15) MINUTE) AS duration,
    CASE WHEN i%5=0 THEN NULL ELSE i*100 END AS sparse_int
FROM generate_series(1, 12) AS t(i);
COPY m5 TO 'test_data/mixed_5.parquet' (FORMAT PARQUET);

-- ============================================================
-- Mixed 6: tinyint, float, varchar, timestamp_ns, fixed array, uuid
-- ============================================================
CREATE OR REPLACE TABLE m6 AS
SELECT
    (i % 100)::TINYINT AS tiny_val,
    (i * 0.707)::FLOAT AS wave,
    concat('record_', lpad(i::VARCHAR, 3, '0'))::VARCHAR AS ref_code,
    ('2024-12-01 00:00:00'::TIMESTAMP_NS + INTERVAL (i) HOUR)::TIMESTAMP_NS AS precise_ts,
    [i, i+1, i+2]::INT[3] AS triple,
    uuid() AS trace_id
FROM generate_series(1, 12) AS t(i);
COPY m6 TO 'test_data/mixed_6.parquet' (FORMAT PARQUET);

-- ============================================================
-- Narrow table: 5 columns, 1M rows
-- ============================================================
CREATE OR REPLACE TABLE narrow_big AS
SELECT
    i::INTEGER AS id,
    (i * 2.71828)::DOUBLE AS value,
    concat('item_', i % 500)::VARCHAR AS category,
    ('2020-01-01'::DATE + INTERVAL (i % 1500) DAY)::DATE AS event_date,
    (i % 2 = 0)::BOOLEAN AS is_active
FROM generate_series(1, 1000000) AS t(i);
COPY narrow_big TO 'test_data/narrow_1m.parquet' (FORMAT PARQUET);

-- ============================================================
-- Wide table: 100 columns, 1000 rows
-- ============================================================
CREATE OR REPLACE TABLE wide_100 AS
SELECT
    i::INTEGER AS id,
    -- 20 integers
    (i * 1)::INTEGER AS int_01, (i * 2)::INTEGER AS int_02, (i * 3)::INTEGER AS int_03,
    (i * 4)::INTEGER AS int_04, (i * 5)::INTEGER AS int_05, (i * 6)::INTEGER AS int_06,
    (i * 7)::INTEGER AS int_07, (i * 8)::INTEGER AS int_08, (i * 9)::INTEGER AS int_09,
    (i * 10)::INTEGER AS int_10, (i * 11)::INTEGER AS int_11, (i * 12)::INTEGER AS int_12,
    (i * 13)::INTEGER AS int_13, (i * 14)::INTEGER AS int_14, (i * 15)::INTEGER AS int_15,
    (i * 16)::INTEGER AS int_16, (i * 17)::INTEGER AS int_17, (i * 18)::INTEGER AS int_18,
    (i * 19)::INTEGER AS int_19, (i * 20)::INTEGER AS int_20,
    -- 20 doubles
    (i * 0.1)::DOUBLE AS dbl_01, (i * 0.2)::DOUBLE AS dbl_02, (i * 0.3)::DOUBLE AS dbl_03,
    (i * 0.4)::DOUBLE AS dbl_04, (i * 0.5)::DOUBLE AS dbl_05, (i * 0.6)::DOUBLE AS dbl_06,
    (i * 0.7)::DOUBLE AS dbl_07, (i * 0.8)::DOUBLE AS dbl_08, (i * 0.9)::DOUBLE AS dbl_09,
    (i * 1.1)::DOUBLE AS dbl_10, (i * 1.2)::DOUBLE AS dbl_11, (i * 1.3)::DOUBLE AS dbl_12,
    (i * 1.4)::DOUBLE AS dbl_13, (i * 1.5)::DOUBLE AS dbl_14, (i * 1.6)::DOUBLE AS dbl_15,
    (i * 1.7)::DOUBLE AS dbl_16, (i * 1.8)::DOUBLE AS dbl_17, (i * 1.9)::DOUBLE AS dbl_18,
    (i * 2.1)::DOUBLE AS dbl_19, (i * 2.2)::DOUBLE AS dbl_20,
    -- 20 varchars
    concat('a_', i % 100) AS str_01, concat('b_', i % 200) AS str_02,
    concat('c_', i % 300) AS str_03, concat('d_', i % 400) AS str_04,
    concat('e_', i % 50) AS str_05, concat('f_', i % 60) AS str_06,
    concat('g_', i % 70) AS str_07, concat('h_', i % 80) AS str_08,
    concat('i_', i % 90) AS str_09, concat('j_', i % 150) AS str_10,
    concat('k_', i % 250) AS str_11, concat('l_', i % 350) AS str_12,
    concat('m_', i % 450) AS str_13, concat('n_', i % 500) AS str_14,
    concat('o_', i % 10) AS str_15, concat('p_', i % 20) AS str_16,
    concat('q_', i % 30) AS str_17, concat('r_', i % 40) AS str_18,
    concat('s_', i % 55) AS str_19, concat('t_', i % 65) AS str_20,
    -- 20 dates
    ('2020-01-01'::DATE + INTERVAL (i % 365) DAY)::DATE AS dt_01,
    ('2021-01-01'::DATE + INTERVAL (i % 365) DAY)::DATE AS dt_02,
    ('2022-01-01'::DATE + INTERVAL (i % 365) DAY)::DATE AS dt_03,
    ('2023-01-01'::DATE + INTERVAL (i % 365) DAY)::DATE AS dt_04,
    ('2024-01-01'::DATE + INTERVAL (i % 365) DAY)::DATE AS dt_05,
    ('2020-06-01'::DATE + INTERVAL (i % 180) DAY)::DATE AS dt_06,
    ('2021-06-01'::DATE + INTERVAL (i % 180) DAY)::DATE AS dt_07,
    ('2022-06-01'::DATE + INTERVAL (i % 180) DAY)::DATE AS dt_08,
    ('2023-06-01'::DATE + INTERVAL (i % 180) DAY)::DATE AS dt_09,
    ('2024-06-01'::DATE + INTERVAL (i % 180) DAY)::DATE AS dt_10,
    ('2020-01-01 00:00:00'::TIMESTAMP + INTERVAL (i * 37) MINUTE)::TIMESTAMP AS ts_01,
    ('2021-01-01 00:00:00'::TIMESTAMP + INTERVAL (i * 53) MINUTE)::TIMESTAMP AS ts_02,
    ('2022-01-01 00:00:00'::TIMESTAMP + INTERVAL (i * 71) MINUTE)::TIMESTAMP AS ts_03,
    ('2023-01-01 00:00:00'::TIMESTAMP + INTERVAL (i * 89) MINUTE)::TIMESTAMP AS ts_04,
    ('2024-01-01 00:00:00'::TIMESTAMP + INTERVAL (i * 97) MINUTE)::TIMESTAMP AS ts_05,
    ('08:00:00'::TIME + INTERVAL (i % 720) MINUTE)::TIME AS tm_01,
    ('12:00:00'::TIME + INTERVAL (i % 360) MINUTE)::TIME AS tm_02,
    ('00:00:00'::TIME + INTERVAL (i % 1440) MINUTE)::TIME AS tm_03,
    ('06:30:00'::TIME + INTERVAL (i % 600) MINUTE)::TIME AS tm_04,
    ('18:00:00'::TIME + INTERVAL (i % 300) MINUTE)::TIME AS tm_05,
    -- 19 booleans / nullables to reach 100
    (i % 2 = 0)::BOOLEAN AS flag_01,
    (i % 3 = 0)::BOOLEAN AS flag_02,
    (i % 5 = 0)::BOOLEAN AS flag_03,
    (i % 7 = 0)::BOOLEAN AS flag_04,
    (i % 11 = 0)::BOOLEAN AS flag_05,
    CASE WHEN i % 3 = 0 THEN NULL ELSE i END AS nullable_01,
    CASE WHEN i % 5 = 0 THEN NULL ELSE i * 2 END AS nullable_02,
    CASE WHEN i % 7 = 0 THEN NULL ELSE concat('n_', i) END AS nullable_03,
    CASE WHEN i % 9 = 0 THEN NULL ELSE (i * 1.1)::DOUBLE END AS nullable_04,
    CASE WHEN i % 11 = 0 THEN NULL ELSE ('2024-01-01'::DATE + INTERVAL (i) DAY)::DATE END AS nullable_05,
    (i * 3 % 1000)::SMALLINT AS small_01,
    (i * 7 % 1000)::SMALLINT AS small_02,
    (i * 11 % 1000)::SMALLINT AS small_03,
    (i * 13 % 1000)::SMALLINT AS small_04,
    (i * 17 % 1000)::SMALLINT AS small_05,
    (i * 19 % 10000)::DECIMAL(10,2) AS dec_01,
    (i * 23 % 10000)::DECIMAL(10,2) AS dec_02,
    (i * 29 % 10000)::DECIMAL(10,2) AS dec_03,
    (i * 31 % 10000)::DECIMAL(10,2) AS dec_04
FROM generate_series(1, 1000) AS t(i);
COPY wide_100 TO 'test_data/wide_100.parquet' (FORMAT PARQUET);

