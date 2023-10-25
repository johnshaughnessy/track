mod api;
mod db;
mod env;
use actix_web::{web, App, HttpServer};
use rusqlite::Connection;
use server::env::load_environment_variables;
use std::sync::Arc;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment_variables(None);

    let db_path = std::env::var("DB_PATH").expect("DB_PATH must be set");
    let conn = Connection::open(db_path).expect("Cannot open database");
    let conn = Arc::new(Mutex::new(conn));

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(conn.clone())).service(
            web::resource("/weights")
                .route(web::post().to(api::add_weight))
                .route(web::get().to(api::get_weights)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests;
