extern crate diesel;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, web, App, HttpServer};
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
    let app_env = std::env::var("APP_ENV").expect("APP_ENV must be set");

    env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = match app_env {
            ref x if x == "dev" => Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            ref x if x == "prod" => Cors::default()
                .allowed_origin("https://www.example.com")
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            _ => panic!("APP_ENV must be set to either 'dev' or 'prod'"),
        };

        println!("APP_ENV: {:?}", std::env::var("APP_ENV"));
        println!("cors: {:?}", cors);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(fs::Files::new("/", "/track/server/static/client/").index_file("index.html"))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/weights")
                            .route(web::get().to(api::get_weights))
                            .route(web::post().to(api::post_weights)),
                    )
                    .service(
                        web::resource("/weights/{weight_id}")
                            .route(web::patch().to(api::patch_weights))
                            .route(web::delete().to(api::delete_weights)),
                    ),
            )
    })
    .bind(format!("{}:{}", ip_address, port))?
    .run()
    .await
}
