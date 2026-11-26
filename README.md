# Pokémon Team Manager

A Rust backend API for managing one Pokémon team of up to six Pokémon.

## Getting started

Requires a Rust toolchain. Check and test the project with:

```text
cargo check
cargo test
```

Run the API locally with:

```text
cargo run
```

The server listens at `http://127.0.0.1:3000`. Available endpoints:

- `GET /health` checks that the API is running.
- `GET /pokemon/{id}` looks up an entry in the local catalog (IDs 1–151).

On startup, the app initializes its SQLite database at `data/team.db`.
