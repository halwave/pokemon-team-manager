pub mod config;
pub mod domain;
pub mod error;
pub mod repo;
pub mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, routes::router()).await
}
