use rusqlite::Row;

use crate::db::SQLiteConnectionPool;

pub struct SQLTable {
    pool: SQLiteConnectionPool,
}

fn row_to_string(row: &Row, col_num: usize) -> String {
    let mut row_str = String::new();

    for col_idx in 0..col_num {
        match row.get_ref(col_idx).unwrap() {
            rusqlite::types::ValueRef::Null => row_str += "NULL|",
            rusqlite::types::ValueRef::Integer(i) => row_str += &(i.to_string() + "|"),
            rusqlite::types::ValueRef::Real(r) => row_str += &(r.to_string() + "|"),
            rusqlite::types::ValueRef::Text(t) => {
                row_str += &(std::str::from_utf8(t).unwrap().to_string() + "|")
            }
            rusqlite::types::ValueRef::Blob(b) => {
                row_str += &(std::str::from_utf8(b).unwrap().to_string() + "|")
            }
        }
    }

    row_str
}

impl SQLTable {
    pub fn new(pool: SQLiteConnectionPool) -> SQLTable {
        SQLTable { pool }
    }

    pub fn get_rows(&self, table: &String) -> Vec<String> {
        let mut rows = Vec::new();
        let conn = self.pool.get().unwrap();
        let mut stmt = conn
            .prepare_cached(format!("SELECT * FROM {}", table).as_str())
            .unwrap();
        let col_num = stmt.column_count();
        let row_iter = stmt
            .query_map([], |row| Ok(row_to_string(row, col_num)))
            .unwrap();

        for row in row_iter {
            let row = row.unwrap();
            rows.push(row);
        }

        rows
    }
}
