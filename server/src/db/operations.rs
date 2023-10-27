use crate::db::CreateWeightPayload;
// use diesel::prelude::*;
use diesel::result::Error;

use diesel::PgConnection;

#[allow(unused)]
pub fn initialize_db(conn: &PgConnection) -> Result<(), Error> {
    //    conn.execute(
    //        "CREATE TABLE IF NOT EXISTS weight (
    //               id INTEGER PRIMARY KEY,
    //               weight REAL,
    //               timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)",
    //        [],
    //    )?;
    Ok(())
}

pub fn save_weight(conn: &PgConnection, weight: &CreateWeightPayload) -> Result<(), Error> {
    //   conn.execute("INSERT INTO weights (weight) VALUES (?1)", params![weight])?;
    Ok(())
}

pub fn get_weights(conn: &PgConnection) -> Result<Vec<(i32, f64, String)>, Error> {
    //  let mut stmt = conn.prepare("SELECT id, weight, timestamp FROM weights")?;
    //  let weight_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
    //  let weights: Result<Vec<(i32, f64, String)>> = weight_iter.collect();
    //weights
    Ok(vec![])
}
