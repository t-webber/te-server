use crate::AddRoutes;
use axum::{http, routing, Json, Router};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct MdlRouter();

impl AddRoutes<MdlRouter> for Router {
    fn add_routes(self, _: PhantomData<MdlRouter>) -> Self {
        self.route("/mdls/get", routing::get(get_mdl))
            .route("/mdls/post", routing::post(create_mdl))
    }
}

async fn create_mdl(Json(payload): Json<CreateMdl>) -> (http::StatusCode, Json<Mdl>) {
    let mdl = Mdl {
        id: 1337,
        mdlname: payload.mdlname,
    };

    (http::StatusCode::CREATED, Json(mdl))
}

async fn get_mdl() -> &'static str {
    "Hello, World!"
}

// the input to our `create_mdl` handler
#[derive(Deserialize)]
struct CreateMdl {
    mdlname: String,
}

/// ## Mail à Diffusion Large
#[derive(Serialize)]
struct Mdl {
    id: u64,
    mdlname: String,
}
