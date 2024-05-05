use crate::AddRoutes;
use axum::{http, routing, Json, Router};
use libs::models::NewUser;
use libs::users;
use std::marker::PhantomData;

pub struct UserRouter();

impl AddRoutes<UserRouter> for Router {
    fn add_routes(self, _: PhantomData<UserRouter>) -> Self {
        self.route("/users/get", routing::get(get_users))
            .route("/users/post", routing::post(create_user))
    }
}

async fn create_user(Json(payload): Json<NewUser>) -> http::StatusCode {
    match users::create(
        payload.email.to_owned(),
        payload.firstname.map(|firstname| firstname.to_owned()),
        payload.lastname.map(|lastname| lastname.to_owned()),
    ) {
        Ok(_) => http::StatusCode::CREATED,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_users() -> String {
    users::get().map_or_else(
        |err| format!("Error {err}: get users failed"),
        |users| format!("Users = {:?}", users),
    )
}
