# High-Level Design (HLD): Pokémon Team Manager API

## 1. Overview & Goals

A lightweight, long-running backend JSON REST API written in Rust that allows authenticated users to manage their single Pokémon team, persisted in SQLite, while isolating storage behind a repository boundary.

### Goals

- Provide a clean, predictable HTTP JSON API for authentication and team management.
- Enforce domain invariants (e.g., team size limits, slot bounds, ownership checks) in core domain/service logic rather than HTTP handlers.
- Keep database drivers and SQL dialects isolated behind repository traits so SQLite can be swapped or supplemented later (e.g., PostgreSQL) without rewriting handlers or domain rules.
- Maintain clear separation of concerns, explicit error types, and straightforward testability.
- Give each user one team with up to six slots. Slots have distinct identities and may reference the same Pokémon.
- Represent Pokémon with a basic ID and name, selected from a small shared list.
- Support adding a Pokémon to an empty slot, removing one, and replacing the Pokémon in an occupied slot.

### Non-Goals (v1)

- Complex battle simulation engines, damage calculators, or full Pokédex game-data scraping.
- Distributed multi-node clustering, message queues, or microservices split.
- OAuth2/OIDC providers, JWT refresh rotation, or role-based access control (RBAC) matrices.

---

## 2. Tech Stack & Dependencies

- **Language:** Rust (edition 2021 or 2024)
- **HTTP & Routing:** `axum` (with `tower` / `tower-http` for middleware such as tracing/logging)
- **Async Runtime:** `tokio` (`full` or `rt-multi-thread` + `macros`)
- **Serialization:** `serde`, `serde_json`
- **Database:** SQLite via `sqlx` (async, compile-time/query-checked) or `rusqlite` (synchronous, minimal overhead)
- **Password Hashing:** `argon2` (or `bcrypt`)
- **Token Generation:** `rand` / `uuid` (cryptographically random opaque session/bearer tokens)
- **Error Handling:** `thiserror` (domain/app errors) and `anyhow` (startup/wiring if needed)

---

## 3. Architecture & Module Layout

```text
src/
  main.rs            # Loads config, initializes DB & repos, builds router, starts server
  config.rs          # Environment/config parsing (bind address, SQLite path/URL)
  error.rs           # Unified AppError enum + IntoResponse for JSON error payloads
  domain.rs          # Core models (User, Team, TeamSlot, Pokemon) + domain rules/validation
  auth.rs            # Login handler, password verification, opaque Bearer token extractor
  routes/
    mod.rs           # Route composition and shared AppState definition
    health.rs        # GET /health endpoint
    auth.rs          # POST /auth/login (and optional POST /auth/register)
    team.rs          # Team read and update handlers
  repo/
    mod.rs           # Repository traits (UserRepository, SessionRepository, TeamRepository)
    sqlite.rs        # SQLite implementations of the repository traits + migrations/init
```
