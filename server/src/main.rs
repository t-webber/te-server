mod data;
use axum::{routing, Router};
use data::DataBaseRouter;
use std::marker::PhantomData;

pub trait AddRoutes<T> {
    fn add_routes(self, _: PhantomData<T>) -> Self;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", routing::get(root))
        .add_routes(PhantomData::<DataBaseRouter>);

    // run our app with hyper, listening globally on port 3000
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
