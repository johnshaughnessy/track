extern crate rusqlite;
use rusqlite::{params, Connection, Result};
use std::env;
extern crate dotenv;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let db_path = env::var("DB_PATH").expect("DB_PATH must be set");

    // Create a SQLite connection
    let conn = Connection::open(db_path)?;

    // Create table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weights (id INTEGER PRIMARY KEY, weight REAL, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)",
        [],
    )?;

    // Insert a weight
    let weight: f64 = 160.2;
    conn.execute("INSERT INTO weights (weight) VALUES (?1)", params![weight])?;

    // Retrieve weights
    let mut stmt = conn.prepare("SELECT id, weight, timestamp FROM weights")?;
    let weight_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    for weight in weight_iter {
        let (id, weight, timestamp): (i32, f64, String) = weight?;
        println!("id: {}, weight: {}, timestamp: {}", id, weight, timestamp);
    }

    Ok(())
}
