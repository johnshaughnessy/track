extern crate rusqlite;
extern crate server;

use rusqlite::{Connection, Error as RusqliteError};
use std::env;
use std::io;
use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
enum TrackServerError {
    #[error("Database error: {0}")]
    Database(#[from] RusqliteError),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),
}

fn main() -> Result<(), TrackServerError> {
    let db_path = match env::var("DB_PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Warning: DB_PATH environment variable not set");
            return Ok(());
        }
    };

    // Check if the database file already exists
    if Path::new(&db_path).exists() {
        eprintln!("Warning: Database file already exists at path: {}", db_path);

        // Assuming it's an SQLite database, print some metadata
        let conn = Connection::open(&db_path)?;
        let db_version: i32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

        eprintln!("Database version: {}", db_version);
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
        let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for table in tables {
            println!("Table: {}", table.unwrap_or_default());
        }
    } else {
        println!("No database found at DB_PATH ({}).", db_path);
    }

    // Confirm if a new database should be created
    eprint!("Would you like to rebuild the database? [y/N]: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() == "y" {
        // Create a new database
        let conn = Connection::open(&db_path)?;
        server::db::initialize_db(&conn)?;
        println!("Successfully created new database at {}", db_path);
    } else {
        println!("Database rebuild cancelled.");
    }
    Ok(())
}
