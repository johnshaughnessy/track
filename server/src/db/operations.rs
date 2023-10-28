use crate::db::schema::weights;
use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};

use super::types::{CreateWeightInput, Weight};

pub fn initialize_db(conn: &mut PgConnection) -> Result<(), Error> {
    Ok(())
}

pub fn create_weight(
    conn: &mut PgConnection,
    payload: &CreateWeightInput,
) -> Result<Weight, Error> {
    let new_weight = Weight {
        weight_id: 0, // Placeholder, assuming it's auto-incremented in the DB
        weight_kg: payload.weight_kg,
        measured_at: payload.measured_at,
        created_at: 0, // Placeholder, to be set by the database if you have set up automatic timestamps
        updated_at: 0, // Placeholder
    };

    let inserted_weight = diesel::insert_into(weights::table)
        .values(&new_weight)
        .returning(weights::all_columns)
        .get_result::<(i64, f64, i64, i64, i64)>(conn)?;

    let inserted_weight = Weight {
        weight_id: inserted_weight.0,
        weight_kg: inserted_weight.1,
        measured_at: inserted_weight.2,
        created_at: inserted_weight.3,
        updated_at: inserted_weight.4,
    };

    Ok(inserted_weight)
}

pub fn list_weights(conn: &mut PgConnection) -> Result<Vec<(i32, f64, String)>, Error> {
    Ok(vec![])
}
