use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub mod notebooks;
pub mod notes;


pub trait CoerceErrExt<T, U : Error> {
    fn coerce_err(self) -> Result<T, Response>;
}

impl<T, U: Error> CoerceErrExt<T, U> for Result<T, U> {
    fn coerce_err(self) -> Result<T, Response> {
        self.map_err(|e|
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        )
    }
}