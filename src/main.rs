#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB Url not found.");
    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Unable to create the connection pool.");

    HttpServer::new(move || App::new()
        .service(hello_world).app_data(pool.clone()))
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await
}
