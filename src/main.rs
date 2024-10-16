#![allow(unused)]

use axum::{response::{Html, IntoResponse}, routing::{get, get_service}, Json, Router};
use tower_http::services::{ServeDir, ServeFile};

mod error;
mod web;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route_service("/", ServeFile::new("templates/index.html"))
        .merge(web::routes_identify::routes())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

