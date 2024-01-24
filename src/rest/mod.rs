use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use schemars::JsonSchema;
use serde::Deserialize;

pub mod notebooks;
pub mod notes;

/// Id path parameter. For use in routes across rest api.
#[derive(Deserialize, JsonSchema)]
pub struct Id {
    pub id: i32
}

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