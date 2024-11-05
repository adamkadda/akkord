#![allow(unused)]

use askama::Template;
use axum::{response::{Html, IntoResponse}, routing::{get, get_service}, Json, Router};
use tower_http::services::{ServeDir, ServeFile};

mod error;
mod web;
mod models;
mod services;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(render_main))
        .route("/about", get(render_about))
        .merge(web::routes_identify::routes())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn render_main() -> impl IntoResponse {
    println!("->> {:<12} - render_main\n", "HANDLER");
    let template = templates::IdentifierTemplate;
    Html(template.render().unwrap())
}

async fn render_about() -> impl IntoResponse {
    println!("->> {:<12} - render_about\n", "HANDLER");
    let template = templates::AboutTemplate;
    Html(template.render().unwrap())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}