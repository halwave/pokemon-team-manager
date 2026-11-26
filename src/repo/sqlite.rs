//! SQLite repository implementation and database initialization.

use std::path::Path;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::domain::{Pokemon, TEAM_SIZE, Team};

use super::{RepositoryError, TeamRepository};

pub struct SqliteTeamRepository {
    pool: SqlitePool,
}

impl SqliteTeamRepository {
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        Self::from_pool(pool).await
    }

    pub async fn in_memory() -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new().filename(":memory:");
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        Self::from_pool(pool).await
    }

    async fn from_pool(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS team_slots (
                slot_id INTEGER PRIMARY KEY CHECK (slot_id BETWEEN 1 AND 6),
                pokemon_id INTEGER NOT NULL CHECK (pokemon_id BETWEEN 1 AND 4294967295),
                pokemon_name TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl TeamRepository for SqliteTeamRepository {
    async fn load_team(&self) -> Result<Team, RepositoryError> {
        let rows = sqlx::query_as::<_, (i64, i64, String)>(
            "SELECT slot_id, pokemon_id, pokemon_name
             FROM team_slots
             ORDER BY slot_id",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut slots: [Option<Pokemon>; TEAM_SIZE] = std::array::from_fn(|_| None);
        for (slot_id, pokemon_id, name) in rows {
            let slot_index = usize::try_from(slot_id)
                .ok()
                .filter(|slot_id| (1..=TEAM_SIZE).contains(slot_id))
                .map(|slot_id| slot_id - 1)
                .ok_or(RepositoryError::InvalidSlotId(slot_id))?;
            let pokemon_id = u32::try_from(pokemon_id)
                .map_err(|_| RepositoryError::InvalidPokemonId(pokemon_id))?;

            slots[slot_index] = Some(Pokemon {
                id: pokemon_id,
                name,
            });
        }

        Ok(Team::from_slots(slots))
    }

    async fn save_team(&self, team: &Team) -> Result<(), RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM team_slots")
            .execute(&mut *transaction)
            .await?;

        for (index, pokemon) in team.slots().iter().enumerate() {
            if let Some(pokemon) = pokemon {
                sqlx::query(
                    "INSERT INTO team_slots (slot_id, pokemon_id, pokemon_name)
                     VALUES (?, ?, ?)",
                )
                .bind((index + 1) as i64)
                .bind(i64::from(pokemon.id))
                .bind(&pokemon.name)
                .execute(&mut *transaction)
                .await?;
            }
        }

        transaction.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn loads_an_empty_team_when_database_has_no_slots() {
        let repository = SqliteTeamRepository::in_memory().await.unwrap();

        assert_eq!(repository.load_team().await.unwrap(), Team::new());
    }

    #[tokio::test]
    async fn saves_and_loads_sparse_team_slots() {
        let repository = SqliteTeamRepository::in_memory().await.unwrap();
        let mut team = Team::new();
        let sample = |id| Pokemon {
            id,
            name: format!("Sample Creature {id:03}"),
        };
        team.add_pokemon(sample(1)).unwrap();
        team.add_pokemon(sample(2)).unwrap();
        team.add_pokemon(sample(1)).unwrap();
        team.remove_pokemon(2).unwrap();

        repository.save_team(&team).await.unwrap();

        assert_eq!(repository.load_team().await.unwrap(), team);
    }

    #[tokio::test]
    async fn team_data_survives_reopening_the_database() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "pokemon-team-manager-{}-{unique_id}.db",
            std::process::id()
        ));

        let mut team = Team::new();
        team.add_pokemon(Pokemon {
            id: 25,
            name: "Sample Creature 025".to_owned(),
        })
        .unwrap();

        let repository = SqliteTeamRepository::connect(&path).await.unwrap();
        repository.save_team(&team).await.unwrap();
        repository.pool.close().await;
        drop(repository);

        let repository = SqliteTeamRepository::connect(&path).await.unwrap();
        assert_eq!(repository.load_team().await.unwrap(), team);
        repository.pool.close().await;
        drop(repository);

        std::fs::remove_file(path).unwrap();
    }
}
