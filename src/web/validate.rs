use axum::{http::HeaderMap, Json};

use crate::error::{Error, Result};
use super::routes_identify::Payload;

pub fn validate_payload(
    headers: HeaderMap,
    Json(payload): Json<Payload>,
) -> Result<Payload> {
    println!("->> {:<12} - validate_payload", "MIDDLEWARE");

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
    
    Ok(payload)
}

pub fn parse_payload(payload: Payload) -> Result<Vec<i8>> {
    let mut notes= Vec::new();

    for str_note in payload.notes {

        let note = match str_note.parse::<i8>() {
            Ok(n) => {
                notes.push(n);
            }
            _ => return Err(Error::ParsingError),
        };
    }
    
    Ok(notes)
}