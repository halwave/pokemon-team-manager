use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{domain::Pokemon, pokemon_catalog::PokemonCatalog};

pub async fn get_pokemon(
    State(catalog): State<PokemonCatalog>,
    Path(id): Path<u32>,
) -> Result<Json<Pokemon>, StatusCode> {
    match catalog.find(id) {
        Some(pokemon) => Ok(Json(pokemon.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
