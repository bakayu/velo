# Velo

A newsletter subscription service built with Rust and actix-web

## Prerequisites

- Rust toolchain (1.70+)
- Docker (for running Postgres)
- `sqlx-cli`: `cargo install sqlx-cli --no-default-features --features rustls,postgres`

## Setup

1. **Start the database**:

   ```bash
   ./scripts/init_db.sh
   ```

2. **Run the application**:

   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8000`.

## API Endpoints

### Health Check

```sh
GET /health_check
```

Returns 200 OK if the service is running.

### Subscribe

```sh
POST /subscribe
Content-Type: application/x-www-form-urlencoded

name=John+Doe&email=john@example.com
```

Subscribes a user to the newsletter. Returns 200 on success, 400 on invalid request and 500 on database error.

## Development

**Run tests**:

```bash
cargo test
```

**Format code**:

```bash
cargo fmt
```

**Lint**:

```bash
cargo clippy
```

**Check dependencies**:

```bash
cargo deny check advisories
```

**Watch mode** (auto-reload on changes):

```bash
cargo watch -x 'clippy --all-features --all-targets -- -D warnings' -x test -x run | bunyan
```

> [!TIP]
> `bunyan` is used to prettify the telemetry logs, you can install it via `npm` or `cargo install bunyan`.

## Configuration

Settings are in `config.yaml`:

- `application_port`: HTTP server port (default: 8000)
- `database`: PostgreSQL connection details: `username`, `password`, `host`, `port`, `database_name`

## Database

The application uses PostgreSQL with the following schema:

- **subscriptions** table:
  - `id` (UUID, primary key)
  - `email` (TEXT, unique)
  - `name` (TEXT)
  - `subscribed_at` (TIMESTAMPTZ)

Migrations are in `migrations/` and run automatically via `sqlx`.

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string (set by `init_db.sh`)
- `TEST_LOG`: Enable logging output for tests
- `SKIP_DOCKER`: Skip Docker container creation in `init_db.sh`

## License

[MIT LICENSE](./LICENSE)
