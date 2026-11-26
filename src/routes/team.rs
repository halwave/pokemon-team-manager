//! Team management endpoints.

use axum::{
    Json,
    extract::{Path, State, rejection::PathRejection},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{Pokemon, TEAM_SIZE, Team},
    error::AppError,
};

use super::AppState;

#[derive(Deserialize)]
pub(super) struct PokemonRequest {
    pokemon_id: u32,
}

#[derive(Serialize)]
pub(super) struct TeamResponse {
    slots: [TeamSlotResponse; TEAM_SIZE],
}

#[derive(Serialize)]
struct TeamSlotResponse {
    slot_id: usize,
    pokemon: Option<Pokemon>,
}

impl From<Team> for TeamResponse {
    fn from(team: Team) -> Self {
        Self {
            slots: std::array::from_fn(|index| TeamSlotResponse {
                slot_id: index + 1,
                pokemon: team.slots()[index].clone(),
            }),
        }
    }
}

pub(super) async fn get_team(
    State(state): State<AppState>,
) -> Result<Json<TeamResponse>, AppError> {
    let _guard = state.team_lock.lock().await;
    let team = state.team_repository.load_team().await?;

    Ok(Json(team.into()))
}

pub(super) async fn add_pokemon(
    State(state): State<AppState>,
    body: Result<Json<PokemonRequest>, axum::extract::rejection::JsonRejection>,
) -> Result<(StatusCode, Json<TeamResponse>), AppError> {
    let Json(request) = body?;
    let _guard = state.team_lock.lock().await;
    let pokemon = state
        .pokemon_catalog
        .find(request.pokemon_id)
        .cloned()
        .ok_or(AppError::InvalidPokemonId(request.pokemon_id))?;
    let mut team = state.team_repository.load_team().await?;
    team.add_pokemon(pokemon)?;
    state.team_repository.save_team(&team).await?;

    Ok((StatusCode::CREATED, Json(team.into())))
}

pub(super) async fn replace_pokemon(
    State(state): State<AppState>,
    path: Result<Path<usize>, PathRejection>,
    body: Result<Json<PokemonRequest>, axum::extract::rejection::JsonRejection>,
) -> Result<Json<TeamResponse>, AppError> {
    let Path(slot_id) = path?;
    let Json(request) = body?;
    let _guard = state.team_lock.lock().await;
    let pokemon = state
        .pokemon_catalog
        .find(request.pokemon_id)
        .cloned()
        .ok_or(AppError::InvalidPokemonId(request.pokemon_id))?;
    let mut team = state.team_repository.load_team().await?;
    team.replace_pokemon(slot_id, pokemon)?;
    state.team_repository.save_team(&team).await?;

    Ok(Json(team.into()))
}

pub(super) async fn remove_pokemon(
    State(state): State<AppState>,
    path: Result<Path<usize>, PathRejection>,
) -> Result<Json<TeamResponse>, AppError> {
    let Path(slot_id) = path?;
    let _guard = state.team_lock.lock().await;
    let mut team = state.team_repository.load_team().await?;
    team.remove_pokemon(slot_id)?;
    state.team_repository.save_team(&team).await?;

    Ok(Json(team.into()))
}
