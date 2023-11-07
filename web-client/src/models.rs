use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
pub type ID = i64;

#[derive(Deserialize, Debug)]
pub struct Weight {
    pub weight_id: ID,
    pub measured_at: NaiveDateTime,
    pub weight_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CreateWeightPayload {
    pub measured_at: NaiveDateTime,
    pub weight_kg: f64,
}
