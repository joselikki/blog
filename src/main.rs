#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

//use serde_json::StreamDeserializer;
use tera::Tera;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPostHandler, Post};
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Unable to connect with DB");

    match web::block(move || posts.load::<Post>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            let mut context = tera::Context::new();
            context.insert("posts", &data);

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &context).unwrap());
        }
        Err(_err) => {
            return HttpResponse::Ok().body("There was an error retrieving the data");
        }
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data::<DbPool>, 
    template_manager: web::Data<tera::Tera>, 
    request : web::Path<String> 
) -> impl Responder {

    let conn = pool.get().expect("Unable to connect with DB");
    let post_slug = request.into_inner();

    match web::block(move || {posts.filter(slug.eq(&post_slug)).load::<Post>(&conn)}).await {

        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            let mut context = tera::Context::new();
            context.insert("post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("blog.html", &context).unwrap())
            
        }
        Err(_err)=> {
            return HttpResponse::Ok().body("Unable to get data");
        }
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Unable to connect with DB");

    match web::block(move || Post::create_post(&conn, &item)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(_err) => HttpResponse::Ok().body("There was an error retrieving the data"),
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

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .data(pool.clone())
            .app_data(web::Data::new(tera))
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run()
    .await
}
