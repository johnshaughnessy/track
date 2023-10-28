extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod api;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let ip_address = std::env::var("IP_ADDRESS").expect("IP_ADDRESS must be set");
    let port = std::env::var("PORT").expect("PORT must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/weights")
                .route(web::post().to(api::create_weight))
                .route(web::get().to(api::get_weights)),
        )
    })
    .bind(format!("{}:{}", ip_address, port))?
    .run()
    .await
}
