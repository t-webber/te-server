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

mod models;
pub mod posts;
mod schema;
