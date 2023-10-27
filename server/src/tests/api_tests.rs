use std::{
    env,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::api;

extern crate serde_derive;
use actix_web::{test, web, App};

use server::env::{load_environment_variables, AppEnvironment};

#[actix_rt::test]
async fn test_get_weights() {
    // load_environment_variables(AppEnvironment::Test);
    // let db_path = env::var("DB_PATH").expect("DB_PATH environment variable not set.");
    // if !Path::new(&db_path).exists() {
    //     panic!("Test database does not exist at path: {}.\n Generate a test database with: \n    APP_ENV=test cargo run --bin gen_db", db_path);
    // }
    // let conn = match Connection::open(&db_path) {
    //     Ok(conn) => Arc::new(Mutex::new(conn)),
    //     Err(_) => panic!("Failed to connect to test database at path: {}", db_path),
    // };
    // let mut app = test::init_service(
    //     App::new()
    //         .app_data(web::Data::new(conn))
    //         .route("/weights", web::get().to(api::get_weights)),
    // )
    // .await;

    // let req = test::TestRequest::get().uri("/weights").to_request();
    // let resp = test::call_service(&mut app, req).await;
    // let status = resp.status();
    // assert!(status.is_success());
}

#[actix_rt::test]
async fn test_add_weight() {
    // load_environment_variables(AppEnvironment::Test);
    // let db_path = env::var("DB_PATH").expect("DB_PATH environment variable not set.");
    // if !Path::new(&db_path).exists() {
    //     panic!("Test database does not exist at path: {}.\n Generate a test database with: \n    APP_ENV=test cargo run --bin gen_db", db_path);
    // }
    // let conn = match Connection::open(&db_path) {
    //     Ok(conn) => conn,
    //     Err(_) => panic!("Failed to connect to test database at path: {}", db_path),
    // };
    // let mutex_conn = Mutex::new(conn);
    // let arc_conn = Arc::new(mutex_conn);

    // // Configure and run the Actix web server for testing
    // let mut app = test::init_service(
    //     App::new()
    //         .app_data(web::Data::new(arc_conn))
    //         .route("/weights", web::post().to(api::add_weight))
    //         .route("/weights", web::get().to(api::get_weights)),
    // )
    // .await;

    // let weight_data = serde_json::json!({"weight": 70.5});
    // let req = test::TestRequest::post()
    //     .uri("/weights")
    //     .set_json(&weight_data)
    //     .to_request();

    // let resp = test::call_service(&mut app, req).await;
    // let status = resp.status();
    // assert!(status.is_success());
}
