use axum::{http::HeaderMap, Json};

use crate::error::{Error, Result};
use super::routes_identify::Payload;

pub async fn validate(
    headers: HeaderMap,
    Json(payload): Json<Payload>,
) -> Result<Payload> {
    // TODO: Validate header
    if let Some(content_type) = headers.get("Content-Type") {
        match content_type.to_str() {
            Ok(value) if value == "application/json" => (),
            _ => return Err(Error::InvalidHeader(
                "Content-Type must be application/json".to_string()
            )),
        }
    } else {
        return Err(Error::InvalidHeader(
            "Missing Content-Type header".to_string()
        ))
    }

    if payload.notes.is_empty() {
        return Err(Error::InvalidPayload(
            "Payload may not be empty".to_string()
        ))
    }

    for note in &payload.notes {
        if note.is_empty() {
            return Err(Error::InvalidPayload(
                "Payload may not contain empty strings".to_string()
            ));
        }
    }
    
    Ok(payload)
}