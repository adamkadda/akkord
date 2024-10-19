use axum::{http::{header, HeaderMap}, response::Html, routing::get, Json, Router};
use serde::Deserialize;

use crate::error::{Error, Result};

use super::validate::{notes_to_i8, validate_payload};

pub fn routes() -> Router {
    Router::new().route("/identify", get(identify_handler))
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub notes: Vec<String>,
}

async fn identify_handler(
    headers: HeaderMap, 
    Json(payload): Json<Payload>
) -> Result<Html<String>> {
    let validated = validate_payload(headers, Json(payload))?;

    // Parse string notes into i8
    let parsed = notes_to_i8(validated);

    // TODO: Implement real chord naming functionality

    Ok(Html(format!(
        "Hello client, I have nothing for you."
    )))
}