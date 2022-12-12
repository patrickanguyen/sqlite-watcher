use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteSchemaError {
    #[error("Invalid Schema Type `{0}`")]
    SchemaTypeError(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchemaType {
    Table,
    Index,
    View,
    Trigger,
}

impl FromStr for SchemaType {
    type Err = SQLiteSchemaError;

    fn from_str(s: &str) -> Result<Self, SQLiteSchemaError> {
        match s {
            "table" => Ok(Self::Table),
            "index" => Ok(Self::Index),
            "view" => Ok(Self::View),
            "trigger" => Ok(Self::Trigger),
            _ => Err(SQLiteSchemaError::SchemaTypeError(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct SQLiteSchema {
    pub schema_type: SchemaType,
    pub name: String,
    pub tbl_name: String,
}

pub type SQLiteConnectionPool = Pool<SqliteConnectionManager>;

pub struct SQLiteSchemaTable {
    pool: SQLiteConnectionPool,
}

impl SQLiteSchemaTable {
    pub fn new(pool: SQLiteConnectionPool) -> SQLiteSchemaTable {
        SQLiteSchemaTable { pool }
    }

    pub fn get_rows(&self) -> Result<Vec<SQLiteSchema>, anyhow::Error> {
        const SCHEMA_SELECT_SQL: &str = r"SELECT type, name, tbl_name FROM sqlite_schema;";

        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare_cached(SCHEMA_SELECT_SQL)?;

        let schema_iter = stmt
            .query_map([], |row| {
                let schema_type = {
                    let schema_str: String = row.get(0)?;
                    SchemaType::from_str(&schema_str).unwrap()
                };

                Ok(SQLiteSchema {
                    schema_type,
                    name: row.get(1)?,
                    tbl_name: row.get(2)?,
                })
            })
            .unwrap();

        let mut rows = Vec::new();

        for schema in schema_iter {
            let schema = schema.unwrap();
            if schema.schema_type == SchemaType::Table {
                rows.push(schema);
            }
        }

        Ok(rows)
    }
}
