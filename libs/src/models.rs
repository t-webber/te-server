use crate::schema::users;
use diesel::{sqlite, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[allow(dead_code)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}
