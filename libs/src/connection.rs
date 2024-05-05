use std::env;

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
/// # Panics
/// If the `.env` is not set correctly.
pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|err| {
        panic!("DATABASE_URL must be set: {err}.\n\n >>> Read the README.md file for more information. <<<")
    });

    SqliteConnection::establish(&database_url).unwrap_or_else(|err| {
        panic!("Error connecting to the database at {database_url}: {err}",);
    })
}
