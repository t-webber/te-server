#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::single_call_fn,
    clippy::mod_module_files,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::missing_inline_in_public_items,
    clippy::question_mark_used
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

mod connection;
pub mod models;
mod schema;
pub mod users;

use std::env;

#[allow(dead_code)]
fn main() {
    let action = env::args().nth(1).unwrap_or_default();
    if action == "show" {
        println!("{:?}", users::get());
    } else if action == "create" {
        if let Some(email) = env::args().nth(2) {
            users::create(email, env::args().nth(3), env::args().nth(4))
                .expect("Error creating user");
            println!("User create ");
        } else {
            println!(
                "Usage: cargo run -- create <email> [<firstname>] [<lastname>]\nPlease provide a non empty title."
            );
        }
    } else {
        println!("Usage: cargo run -- <show|create [<firstname>] [<lastname>]>");
    }
}
