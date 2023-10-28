use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{Float8, Int8};
use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::schema::weights;

use super::{Timestamp, ID};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::weights)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Weight {
    #[diesel(sql_type = Int8)]
    pub weight_id: ID,
    #[diesel(sql_type = Int8)]
    #[serde(skip_serializing)]
    pub inserted_at: Timestamp,
    #[diesel(sql_type = Int8)]
    #[serde(skip_serializing)]
    pub updated_at: Timestamp,
    #[diesel(sql_type = Int8)]
    pub measured_at: Timestamp,
    #[diesel(sql_type = Float8)]
    pub weight_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::weights)]
pub struct CreateWeightPayload {
    pub measured_at: Timestamp,
    pub weight_kg: f64,
}

pub fn create_weight(
    conn: &mut PgConnection,
    create_weight_payload: &CreateWeightPayload,
) -> Result<Weight, diesel::result::Error> {
    let inserted_weight = diesel::insert_into(weights::table)
        .values(create_weight_payload)
        .returning(weights::all_columns)
        .get_result(conn)?;

    println!("create_weight results: \n{:?}", inserted_weight);
    Ok(inserted_weight)
}

pub fn list_weights(conn: &mut PgConnection) -> Result<Vec<Weight>, diesel::result::Error> {
    use crate::schema::weights::dsl::*;

    let results: Result<Vec<Weight>, diesel::result::Error> = weights.limit(100).load(conn);

    println!("list_weights results: \n{:?}", results);

    results
}
