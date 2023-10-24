use rusqlite::{params, Connection, Result};
use std::env;
extern crate dotenv;

fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weights (id INTEGER PRIMARY KEY, weight REAL, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)",
        [],
    )?;
    Ok(())
}

fn save_weight(conn: &Connection, weight: f64) -> Result<()> {
    conn.execute("INSERT INTO weights (weight) VALUES (?1)", params![weight])?;
    Ok(())
}

fn get_weights(conn: &Connection) -> Result<Vec<(i32, f64, String)>> {
    let mut stmt = conn.prepare("SELECT id, weight, timestamp FROM weights")?;
    let weight_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    let weights: Result<Vec<(i32, f64, String)>> = weight_iter.collect();
    weights
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_save_weight() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_db(&conn).unwrap();
        assert!(save_weight(&conn, 70.5).is_ok());
    }

    #[test]
    fn test_get_weights() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_db(&conn).unwrap();
        save_weight(&conn, 70.5).unwrap();
        let weights = get_weights(&conn).unwrap();
        let (id, weight, timestamp) = &weights[0];
        assert_eq!(*id, 1);
        assert_eq!(*weight, 70.5);

        // Validate the timestamp
        assert!(NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S").is_ok());
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let db_path = env::var("DB_PATH").expect("DB_PATH must be set");
    let conn = Connection::open(db_path)?;
    initialize_db(&conn)?;

    Ok(())
}
