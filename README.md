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
- `GET /team` returns the six team slots.
- `POST /team/slots` adds a Pokémon to the first empty slot. Send `{"pokemon_id": 25}`.
- `PUT /team/slots/{slot_id}` replaces a Pokémon in an occupied slot using the same body.
- `DELETE /team/slots/{slot_id}` removes a Pokémon from a slot.

On startup, the app initializes its SQLite database at `data/team.db`.

For manual API testing, import [`insomnia.collection.json`](./insomnia.collection.json) into Insomnia.
