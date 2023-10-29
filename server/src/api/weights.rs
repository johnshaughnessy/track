use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use server::models::{CreateWeightPayload, UpdateWeightPayload, ID};

pub async fn post_weights(
    create_weight_payload: web::Json<CreateWeightPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let create_weight_payload = create_weight_payload.into_inner();
    match server::models::weights::create_weight(&mut conn, &create_weight_payload) {
        Ok(weight) => HttpResponse::Ok().body(format!("Weight added: {:?}", weight)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding weight."),
    }
}

pub async fn get_weights(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match server::models::weights::read_weights(&mut conn) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}

pub async fn patch_weights(
    path: web::Path<(ID,)>,
    update_weight_payload: web::Json<UpdateWeightPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let target_weight_id: ID = path.0;
    let update_weight_payload = update_weight_payload.into_inner();

    match server::models::weights::update_weight(
        &mut conn,
        target_weight_id,
        &update_weight_payload,
    ) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}

pub async fn delete_weights(
    path: web::Path<(ID,)>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let target_weight_id: ID = path.0;

    match server::models::weights::delete_weight(&mut conn, target_weight_id) {
        Ok(weights) => HttpResponse::Ok().json(weights),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving weights."),
    }
}
