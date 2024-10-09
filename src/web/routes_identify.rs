use axum::{http::{header, HeaderMap}, response::Html, routing::get, Json, Router};
use serde::Deserialize;

use crate::error::{Error, Result};

use super::validate::validate;

pub fn routes() -> Router {
    Router::new().route("/identify", get(identify))
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub notes: Vec<String>,
}

async fn identify(
    headers: HeaderMap, 
    Json(payload): Json<Payload>
) -> Result<Html<String>> {
    let validated_payload = validate(headers, Json(payload)).await?;

    // TODO: Implement real chord naming functionality

    Ok(Html(format!(
        "Business logic executed with payload: {:?}",
        validated_payload.notes
    )))
}
