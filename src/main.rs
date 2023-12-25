mod controllers;
mod helpers;

use axum::{http::StatusCode, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let app = Router::new()
        .nest("/execute", controllers::executor::router())
        .route("/", get(root))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::FOUND, "Hello rust")
}
