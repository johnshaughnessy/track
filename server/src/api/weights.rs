use std::sync::{Arc, Mutex};

use crate::db;
use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use serde_derive::Deserialize;
extern crate serde_derive;

#[derive(Deserialize)]
pub struct WeightPayload {
    weight: f64,
}

pub async fn add_weight(
    weight: web::Json<WeightPayload>,
    db_conn: web::Data<Arc<Mutex<Connection>>>,
) -> impl Responder {
    let db_conn = db_conn.get_ref().lock().unwrap();
    match db::save_weight(&db_conn, weight.weight) {
        Ok(_) => HttpResponse::Ok().body(format!("Weight added: {}", weight.weight)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding weight."),
    }
}

pub async fn get_weights(db_conn: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let db_conn = db_conn.get_ref().lock().unwrap();
    match db::get_weights(&db_conn) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}
