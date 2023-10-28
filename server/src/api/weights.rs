use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use server::db::operations;
use server::db::types::CreateWeightInput;

pub async fn create_weight(
    input: web::Json<CreateWeightInput>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let payload = input.into_inner();
    match operations::create_weight(&mut conn, &payload) {
        Ok(_) => HttpResponse::Ok().body(format!("Weight added: {}", payload.weight_kg)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding weight."),
    }
}

pub async fn get_weights(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match operations::list_weights(&mut conn) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}
