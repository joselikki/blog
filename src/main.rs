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

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl;

#[get("/")]
async fn index(pool: web ::Data<DbPool>) -> impl Responder {

    let conn = pool.get().expect("Unable to connect with DB");

    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => HttpResponse::Ok().body("Data Retreived"),
        Err(err) => HttpResponse::Ok().body("There was an error retrieving the data")
    }    
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
        .service(index).app_data(pool.clone()))
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await
}
