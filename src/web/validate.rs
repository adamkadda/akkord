use axum::{http::HeaderMap, Json};

use crate::error::{Error, Result};
use super::routes_identify::Payload;

pub fn validate_payload(
    headers: HeaderMap,
    Json(payload): Json<Payload>,
) -> Result<Payload> {
    // TODO: Validate header
    if let Some(content_type) = headers.get("Content-Type") {
        match content_type.to_str() {
            Ok(value) if value == "application/json" => (),
            _ => return Err(Error::InvalidHeader),
        }
    } else {
        return Err(Error::InvalidHeader)
    }

    if payload.notes.is_empty() {
        return Err(Error::InvalidPayload)
    }

    for note in &payload.notes {
        if note.is_empty() {
            return Err(Error::InvalidPayload)
        }
    }
    
    Ok(payload)
}

pub fn notes_to_i8(payload: Payload) -> Result<Vec<i8>> {
    let mut i8_notes= Vec::new();

    for str_note in payload.notes {
        i8_notes.push(str_note.parse().unwrap());
    }
    
    Ok(i8_notes)
}