use askama::Template;
use axum::{http::{header, HeaderMap, StatusCode}, response::{Html, IntoResponse}, routing::{post}, Json, Router};
use serde::Deserialize;

use crate::{error::{Error, Result}, services::identify::identify, templates::ResultTemplate, web::validate::{parse_payload, validate_payload}};

pub fn routes() -> Router {
    Router::new().route("/identify", post(identify_handler))
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub notes: Vec<String>,
}

async fn identify_handler(
    headers: HeaderMap, 
    Json(payload): Json<Payload>
) -> impl IntoResponse {
    println!("->> {:<12} - identify_handler", "HANDLER");
    println!("->> {:<12} - {:?}", "PAYLOAD", &payload);

    let validated = match validate_payload(headers, Json(payload)) {
    Ok(p) => p,
    Err(e) => return e.into_response(),
    };

    let parsed = match parse_payload(validated) {
        Ok(vec) => vec,
        Err(e) => return e.into_response(),
    };

    // dbg!(format!("{:?}", &parsed));

    let chords = match identify(parsed) {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let template = ResultTemplate { chords };
    let response_string = template.render().unwrap();

    println!("->> {:<12} - 200 OK", "INTO_RES");
    println!();
    (StatusCode::OK, Html(response_string)).into_response()
}