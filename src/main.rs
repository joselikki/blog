#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB URL not found");
    let conn = PgConnection::establish(&db_url).expect("Error: Unable to connecto to data base");

    use self::models::{NewPost, Post, PostSimplified};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    // let new_post = NewPost {
    //     title: "Third Blog Post",
    //     body: "3 This is the body of the blobpost",
    //     slug: "third-blog-post",
    // };

    // diesel::insert_into(posts::table)
    //     .values(new_post)
    //     .get_result::<Post>(&conn)
    //     .expect("Falied: Data insertion failed");

    //diesel::update(posts.filter(id.eq(3))).set(slug.eq("post-modified")).get_result::<Post>(&conn).expect("Unable to modify post");

    //let post_del = diesel::delete(posts.filter(id.eq(3))).get_result::<Post>(&conn).expect("Unable to remove post");
    
    println!("Post removed: {}", post_del.title);

    // Get all posts
    let res_posts = posts
        .limit(10)
        .load::<Post>(&conn)
        .expect("Error: Unable to retrieve posts.");

    for post in res_posts {
        println!(
            "id: {} - Title:{} - Slug: {}",
            post.id, post.title, post.slug
        )
    }

    let post_titles = posts
        .select((title, id))
        .load::<PostSimplified>(&conn)
        .expect("Unable to retrieve posts");
    println!("Just titles and ids");
    for post in post_titles {
        println!("{}", post.title);
    }
}
