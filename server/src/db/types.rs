extern crate serde_derive;
use serde_derive::Deserialize;

pub type Timestamp = i32;

#[derive(Deserialize)]
pub struct CreateWeightPayload {
    pub weight_kg: f64,
    pub measured_at: Timestamp,
}
