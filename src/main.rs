#![allow(unused)]

use axum::{response::{Html, IntoResponse}, routing::{get, get_service}, Json, Router};
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .merge(web::routes_identify::routes())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    println!("->> {:<12} - handler_index", "HANDLER");

    Html("hello <strong>world!</strong>")
}


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

