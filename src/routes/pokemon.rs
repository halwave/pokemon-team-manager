use axum::{
    Json,
    extract::{Path, State, rejection::PathRejection},
};

use super::AppState;
use crate::{domain::Pokemon, error::AppError};

pub async fn get_pokemon(
    State(state): State<AppState>,
    path: Result<Path<u32>, PathRejection>,
) -> Result<Json<Pokemon>, AppError> {
    let Path(id) = path?;
    let pokemon = state
        .pokemon_catalog
        .find(id)
        .cloned()
        .ok_or(AppError::PokemonNotFound(id))?;

    Ok(Json(pokemon))
}
