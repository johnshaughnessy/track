use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use server::models::CreateWeightPayload;

pub async fn create_weight(
    create_weight_payload: web::Json<CreateWeightPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let create_weight_payload = create_weight_payload.into_inner();
    match server::models::weight::create_weight(&mut conn, &create_weight_payload) {
        Ok(weight) => HttpResponse::Ok().body(format!("Weight added: {:?}", weight)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding weight."),
    }
}

pub async fn get_weights(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match server::models::weight::list_weights(&mut conn) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}
