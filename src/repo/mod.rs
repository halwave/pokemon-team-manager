//! Repository interfaces and persistence implementations.

use crate::domain::Team;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("invalid stored slot id: {0}")]
    InvalidSlotId(i64),
    #[error("invalid stored Pokémon id: {0}")]
    InvalidPokemonId(i64),
}

#[async_trait::async_trait]
pub trait TeamRepository: Send + Sync {
    async fn load_team(&self) -> Result<Team, RepositoryError>;
    async fn save_team(&self, team: &Team) -> Result<(), RepositoryError>;
}

pub mod sqlite;
