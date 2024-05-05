use crate::AddRoutes;
use axum::{http, routing, Json, Router};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct UserRouter();

impl AddRoutes<UserRouter> for Router {
    fn add_routes(self, _: PhantomData<UserRouter>) -> Self {
        self.route("/users/get", routing::get(get_user))
            .route("/users/post", routing::post(create_user))
    }
}

async fn create_user(Json(payload): Json<CreateUser>) -> (http::StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (http::StatusCode::CREATED, Json(user))
}

async fn get_user() -> &'static str {
    "Hello, World!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
