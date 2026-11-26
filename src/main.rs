use std::sync::Arc;

pub mod config;
pub mod domain;
pub mod error;
pub mod pokemon_catalog;
pub mod repo;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pokemon_catalog = pokemon_catalog::PokemonCatalog::load()?;
    let team_repository =
        Arc::new(repo::sqlite::SqliteTeamRepository::connect("data/team.db").await?);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, routes::router(pokemon_catalog, team_repository)).await?;
    Ok(())
}
