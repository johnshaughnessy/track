use diesel::deserialize::QueryableByName;
use diesel::sql_types::{Float8, Int8};
use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use super::schema::weights;

pub type Timestamp = i64;
pub type ID = i64;

#[derive(Serialize, Deserialize, QueryableByName, Insertable)]
#[diesel(table_name = weights)] // Make sure the table name matches your actual table name
pub struct Weight {
    #[diesel(sql_type = Int8)]
    pub weight_id: ID,
    #[diesel(sql_type = Float8)]
    pub weight_kg: f64,
    #[diesel(sql_type = Int8)]
    pub measured_at: Timestamp,
    #[diesel(sql_type = Int8)]
    #[serde(skip_serializing)]
    pub created_at: Timestamp,
    #[diesel(sql_type = Int8)]
    #[serde(skip_serializing)]
    pub updated_at: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWeightInput {
    pub weight_kg: f64,
    pub measured_at: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateWeightInput {
    pub weight_id: f64,
    pub weight_kg: Option<f64>,
    pub measured_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteWeightInput {
    pub weight_id: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteWeightOutput {
    pub success: bool,
    pub weight_id: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ListWeightsOutput {
    pub weights: Vec<Weight>,
}
