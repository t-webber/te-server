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
    clippy::single_call_fn,
    clippy::mod_module_files
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

use std::env;

// modules
mod models;
mod posts;
mod schema;

use posts::users::{publish_post, show_posts, write_post};

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
