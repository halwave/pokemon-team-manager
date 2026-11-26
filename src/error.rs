//! Application errors and their HTTP responses.

use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{domain::TeamError, repo::RepositoryError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid JSON request body; expected a pokemon_id number")]
    InvalidJsonBody,
    #[error("Invalid path parameter")]
    InvalidPathParameter,
    #[error("Pokemon ID {0} is not in the catalog")]
    InvalidPokemonId(u32),
    #[error("Pokemon ID {0} was not found")]
    PokemonNotFound(u32),
    #[error("The team is full")]
    TeamFull,
    #[error("Invalid slot ID {0}; expected a value from 1 to 6")]
    InvalidSlotId(usize),
    #[error("Team slot {0} is empty")]
    EmptySlot(usize),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

impl From<TeamError> for AppError {
    fn from(error: TeamError) -> Self {
        match error {
            TeamError::Full => Self::TeamFull,
            TeamError::InvalidSlotId(slot_id) => Self::InvalidSlotId(slot_id),
            TeamError::EmptySlot(slot_id) => Self::EmptySlot(slot_id),
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(_: JsonRejection) -> Self {
        Self::InvalidJsonBody
    }
}

impl From<PathRejection> for AppError {
    fn from(_: PathRejection) -> Self {
        Self::InvalidPathParameter
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::InvalidJsonBody
            | Self::InvalidPathParameter
            | Self::InvalidPokemonId(_)
            | Self::InvalidSlotId(_) => StatusCode::BAD_REQUEST,
            Self::PokemonNotFound(_) | Self::EmptySlot(_) => StatusCode::NOT_FOUND,
            Self::TeamFull => StatusCode::CONFLICT,
            Self::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if let Self::Repository(error) = &self {
            eprintln!("Team repository error: {error}");
        }

        let error = match self {
            Self::Repository(_) => "Internal server error".to_owned(),
            other => other.to_string(),
        };

        (status, Json(ErrorResponse { error })).into_response()
    }
}
