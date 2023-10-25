use rusqlite::{params, Connection, Result};

pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weights (id INTEGER PRIMARY KEY, weight REAL, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)",
        [],
    )?;
    Ok(())
}

pub fn save_weight(conn: &Connection, weight: f64) -> Result<()> {
    conn.execute("INSERT INTO weights (weight) VALUES (?1)", params![weight])?;
    Ok(())
}

pub fn get_weights(conn: &Connection) -> Result<Vec<(i32, f64, String)>> {
    let mut stmt = conn.prepare("SELECT id, weight, timestamp FROM weights")?;
    let weight_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    let weights: Result<Vec<(i32, f64, String)>> = weight_iter.collect();
    weights
}
