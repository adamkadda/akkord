use axum::{http::StatusCode, response::{Html, IntoResponse}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidHeader(String),
    InvalidPayload(String),
}


impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let (status, message) = match self {
            Error::InvalidHeader(msg) => (StatusCode::NO_CONTENT, msg),
            Error::InvalidPayload(msg) => (StatusCode::NO_CONTENT, msg),
        };

        (status, Html(format!("Error: {}", message))).into_response()
    }
}


// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate