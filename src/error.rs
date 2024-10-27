use axum::{http::StatusCode, response::{Html, IntoResponse}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidHeader,
    InvalidPayload,

    // Processing errors
    InvalidNotes,
    ParsingError,
    InvalidInterval,
    InvalidInversion,
    InvalidInversionMin3Aug5,
    MissingFifth,
    InvalidTriad,
    InvalidRoot,
}


impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        println!();

        StatusCode::NO_CONTENT.into_response()
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