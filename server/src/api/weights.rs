use crate::db::{self, CreateWeightPayload};
use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub async fn add_weight(
    weight: web::Json<CreateWeightPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let weight = weight.into_inner();
    match db::save_weight(&conn, &weight) {
        Ok(_) => HttpResponse::Ok().body(format!("Weight added: {}", weight.weight_kg)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding weight."),
    }
}

pub async fn get_weights(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();

    match db::get_weights(&conn) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}
