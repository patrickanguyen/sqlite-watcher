mod app;
mod db;

use crate::app::run;
use crate::db::SQLiteSchemaTable;
use anyhow::Result;
use clap::Parser;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use std::time::Duration;

/// SQLite Watcher TUI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to SQLite Database
    db_path: String,
    /// Tick rate in milliseconds
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64

}

fn main() -> Result<()> {
    let args = Args::parse();

    let db_path = Path::new(&args.db_path);

    if !(db_path.exists() && db_path.is_file()) {
        eprintln!(
            "Database `{}` is not a file or does not exist",
            args.db_path
        );
        std::process::exit(1);
    }

    let manager = SqliteConnectionManager::file(db_path)
        .with_flags(rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY);
    let pool = r2d2::Pool::new(manager)?;

    let schema_table = SQLiteSchemaTable::new(pool.clone());
    let tables = schema_table.get_rows()?;

    let tick_rate = Duration::from_millis(args.tick_rate);
    run(tick_rate, pool, tables)?;

    Ok(())
}
