#![allow(unused)]

use askama::Template;
use axum::{body::Body, http::{header, request, HeaderValue, Request}, middleware::{self, Next}, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use tower::{layer, ServiceBuilder};
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
    println!("->> {:<12} - render_main", "HANDLER");
    let template = templates::IdentifierTemplate;
    Html(template.render().unwrap())
}

async fn render_about() -> impl IntoResponse {
    println!("->> {:<12} - render_about", "HANDLER");
    let template = templates::AboutTemplate;
    Html(template.render().unwrap())
}

async fn set_cache_control(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
    );
    response
}

fn routes_static() -> Router {
    Router::new()
        .nest_service(
            "/",
            ServiceBuilder::new()
                .layer(middleware::from_fn(set_cache_control))
                .service(ServeDir::new("./"))
        )
}