# High-Level Design (HLD): Pokémon Team Manager API

## 1. Overview & Goals

A lightweight, long-running backend JSON REST API written in Rust for managing one Pokémon team, persisted in SQLite, while isolating storage behind a repository boundary.

### Goals

- Provide a clean, predictable HTTP JSON API for team management.
- Enforce domain invariants (e.g., team size limits and slot bounds) in core domain logic rather than HTTP handlers.
- Keep database drivers and SQL dialects isolated behind repository traits so SQLite can be swapped or supplemented later (e.g., PostgreSQL) without rewriting handlers or domain rules.
- Maintain clear separation of concerns, explicit error types, and straightforward testability.
- Manage one team with up to six slots. Slots have distinct identities and may reference the same Pokémon.
- Represent Pokémon with a basic ID and name, selected from a small shared list.
- Support adding a Pokémon to an empty slot, removing one, and replacing the Pokémon in an occupied slot.

### Non-Goals (v1)

- Complex battle simulation engines, damage calculators, or full Pokédex game-data scraping.
- Distributed multi-node clustering, message queues, or microservices split.
- User accounts and authentication.

---

## 2. Tech Stack & Dependencies

- **Language:** Rust (edition 2021 or 2024)
- **HTTP & Routing:** `axum` (with `tower` / `tower-http` for middleware such as tracing/logging)
- **Async Runtime:** `tokio` (`full` or `rt-multi-thread` + `macros`)
- **Serialization:** `serde`, `serde_json`
- **Database:** SQLite via `sqlx` (async, compile-time/query-checked) or `rusqlite` (synchronous, minimal overhead)
- **Error Handling:** `thiserror` (domain/app errors) and `anyhow` (startup/wiring if needed)

---

## 3. Architecture & Module Layout

```text
src/
  main.rs            # Loads config, initializes DB & repos, builds router, starts server
  config.rs          # Environment/config parsing (bind address, SQLite path/URL)
  error.rs           # Unified AppError enum + IntoResponse for JSON error payloads
  domain.rs          # Team and Pokemon models; slot positions are numbered 1-6
  routes/
    mod.rs           # Route composition
    health.rs        # GET /health endpoint
    team.rs          # Team read and update handlers
  repo/
    mod.rs           # TeamRepository trait
    sqlite.rs        # SQLite implementation and migrations/init
```
