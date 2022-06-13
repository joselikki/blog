#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB URL not found");
    let conn = PgConnection::establish(&db_url).expect("Error: Unable to connecto to data base");

    use self::models::Post;
    use self::schema::posts::dsl::*;

    let res_posts = posts.load::<Post>(&conn).expect("Error: Unable to retrieve posts.");


    for post in res_posts {
        println!("{}", post.title)
    }
}
