# rust-pastebin

Async REST API for creating and sharing text pastes — built in Rust with a clear layered architecture, typed SQL, and OpenAPI docs out of the box.

A portfolio backend focused on production-style practices: separation of concerns, background jobs, structured errors, and observability — not just CRUD.

---

## Tech stack

| Layer | Choice | Role |
| --- | --- | --- |
| Runtime | Tokio | Async I/O and background tasks |
| HTTP | Actix Web 4 | High-performance REST API |
| Database | PostgreSQL + SQLx | Connection pool, migrations, compile-time SQL |
| IDs | UUID v7 | Time-sortable unique identifiers |
| Docs | utoipa + Swagger UI | OpenAPI from Rust types |
| Errors | thiserror | Typed domain/API errors |
| Logging | tracing | Request-scoped instrumentation |
| Config | dotenvy | Environment-based configuration |

---

## Features

- **Create pastes** with title, content, and language hint
- **Optional TTL** — `expires_in_hours` (default: 30 days)
- **One-time pastes** — deleted after the first successful read
- **View counter** — incremented atomically on read
- **Pagination** — `limit` / `offset` for listing
- **Partial updates** — `PATCH` with coalesce semantics
- **Background cleanup** — expired rows removed every 10 minutes
- **Auto migrations** — applied on startup via SQLx
- **Interactive API docs** — Swagger UI at `/swagger/`

---

## Project structure

```
src/
├── main.rs                 # Server bootstrap, cleanup task, Swagger
├── lib.rs                  # Module tree
├── handlers/               # Actix route handlers
├── services/               # PasteService, CleanUpService
├── repositories/           # SQLx queries
├── models/                 # DTOs & response types
├── db/                     # PgPool + migrations
├── error.rs                # AppError (thiserror + ResponseError)
├── env.rs                  # HOST / PORT / DATABASE_URL
└── types.rs                # Shared aliases (Database, AppResult, Id)
migrations/                 # SQLx SQL migrations
```

---

## Getting started

### Prerequisites

- Rust (edition 2024 toolchain)
- PostgreSQL
- [sqlx-cli](https://github.com/launchbadge/sqlx) (optional, for manual migrations)

### Configuration

Copy env vars into `.env` (gitignored):

```env
HOST=localhost
PORT=8080
DATABASE_URL=postgresql://user:password@localhost:5432/pastebin
```

### Run

```bash
# Create DB (example)
createdb pastebin

# Start API — migrations run automatically on boot
cargo run
```

Server listens on `HOST:PORT`. Open Swagger at `/swagger/`.

---

## What I’d improve next

Honest backlog — useful both as a learning path and as talking points in an interview:

- [ ] Rate limiting and request size limits
- [ ] Integration tests against a test Postgres
- [ ] Docker Compose for one-command local setup

---

## License

MIT (or as preferred).
