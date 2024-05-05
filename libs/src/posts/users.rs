use crate::models::{NewPost, Post};
use crate::schema::posts::{self, dsl};

// standard
use std::env;

// dependencies
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|err| {
        panic!("DATABASE_URL must be set: {err}.\n\n >>> Read the README.md file for more information. <<<")
    });

    SqliteConnection::establish(&database_url).unwrap_or_else(|err| {
        panic!("Error connecting to the database at {database_url}: {err}",);
    })
}

fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts::table
        .order(posts::id.desc())
        .first(conn)
        .unwrap_or_else(|err| {
            panic!("Error loading posts: {err}");
        })
}

pub fn write_post(title: &str, body: &str) {
    let mut conn = establish_connection();
    let post = create_post(&mut conn, title, body);
    println!("Saved post {title} with id {}", post.id);
}

pub fn show_posts() {
    let mut conn = establish_connection();
    let results = dsl::posts
        .filter(dsl::published.eq(true))
        .limit(5)
        .load::<(i32, String, String, bool)>(&mut conn)
        .unwrap_or_else(|err| panic!("Error loading posts: {err}"));

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("\n>>><<<\n{post:?}");
    }
}

pub fn publish_post(post_id: &str) {
    let parsed_id = post_id
        .parse::<i32>()
        .unwrap_or_else(|err| panic!("Invalid ID: {err}"));
    let mut conn = establish_connection();
    diesel::update(dsl::posts.find(parsed_id))
        .set(dsl::published.eq(true))
        .execute(&mut conn)
        .expect("Error publishing post");
}
