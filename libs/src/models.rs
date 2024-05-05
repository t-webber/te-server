use crate::schema::posts;
use diesel::{sqlite, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'post_creation> {
    pub title: &'post_creation str,
    pub body: &'post_creation str,
}
