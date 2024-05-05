use crate::connection::connect_db;
use crate::models::{NewUser, User};
use crate::schema::users::{self, dsl};
use diesel::{result, QueryDsl, RunQueryDsl};

/// # Errors
/// If the insert operation fails.
pub fn create(
    email: String,
    firstname: Option<String>,
    lastname: Option<String>,
) -> Result<(), result::Error> {
    let mut conn = connect_db();
    let new_post = NewUser {
        email,
        firstname,
        lastname,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .execute(&mut conn)?;
    Ok(())
}

/// # Errors
/// If the select operation fails.
pub fn get() -> Result<Vec<User>, result::Error> {
    let mut conn = connect_db();
    dsl::users.limit(10).load::<User>(&mut conn)
}
