#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo // activate to publish
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::single_call_fn
)]
// dev rules (disable for production)
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::panic,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::use_debug,
    clippy::expect_used
)]

// modules
pub mod models;
pub mod schema;
use models::{NewPost, Post};
use schema::posts::{self, dsl};

// standard
use std::env;

// dependencies
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|err| {
        panic!("DATABASE_URL must be set: {err}.\n\n >>> Read the README.md file for more information. <<<")
    });

    SqliteConnection::establish(&database_url).unwrap_or_else(|err| {
        panic!("Error connecting to the database at {database_url}: {err}",);
    })
}

fn show_posts() {
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

fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    // Fetch the last inserted post
    posts::table
        .order(posts::id.desc())
        .first(conn)
        .unwrap_or_else(|err| {
            panic!("Error loading posts: {err}");
        })
}

fn write_post(title: &str, body: &str) {
    let mut conn = establish_connection();
    let post = create_post(&mut conn, title, body);
    println!("Saved post {title} with id {}", post.id);
}

fn publish_post(post_id: &str) {
    let parsed_id = post_id
        .parse::<i32>()
        .unwrap_or_else(|err| panic!("Invalid ID: {err}"));
    let mut conn = establish_connection();
    diesel::update(dsl::posts.find(parsed_id))
        .set(dsl::published.eq(true))
        .execute(&mut conn)
        .expect("Error publishing post");
}

fn main() {
    let action = env::args().nth(1).unwrap_or_default();
    if action == "show" {
        show_posts();
    } else if action == "create" {
        if let Some(title) = env::args().nth(2) {
            if let Some(body) = env::args().nth(3) {
                write_post(&title, &body);
                println!("New post created");
                show_posts();
            } else {
                println!(
                    "Usage: cargo run -- create <title> <body>\nPlease provide a non empty body."
                );
            }
        } else {
            println!(
                "Usage: cargo run -- create <title> <body>\nPlease provide a non empty title."
            );
        }
    } else if action == "publish" {
        if let Some(post_id) = env::args().nth(2) {
            publish_post(&post_id);
            println!("Post published");
            show_posts();
        } else {
            println!("Usage: cargo run -- publish <post_id>");
        }
    } else {
        println!("Usage: cargo run -- [show|publish <post_id>|create <title> <body>]");
    }
}
