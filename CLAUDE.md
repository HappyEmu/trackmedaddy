# CLAUDE.md

## Project Overview

Everhour CLI (`everhour`) — a Rust CLI tool for interacting with the Everhour time tracking API. Primary use case: start/stop timers on tasks that correspond to Linear tickets (e.g. `TRG-80`).

## Build & Run

```
cargo build                  # build debug binary
cargo build --release        # build release binary
./target/debug/everhour      # run debug binary
```

## Project Structure

```
src/
  main.rs       -- CLI parsing (clap derive), subcommand dispatch, command handlers
  config.rs     -- Config load/save (OS config dir, e.g. ~/Library/Application Support/everhour/config.toml on macOS)
  api.rs        -- Everhour API client (reqwest async)
  models.rs     -- Shared data types (Config, Task, Timer, etc.)
```

## Key Conventions

- All API calls go through `EverhourClient` in `api.rs`
- Config is a simple TOML file with `api_key` field
- CLI uses clap derive API with a `Cli` struct and `Command` enum
- Async runtime: `tokio` with `current_thread` flavor
- Error handling: `anyhow::Result` throughout, with `.context()` for user-facing messages

## Everhour API

- Base URL: `https://api.everhour.com`
- Auth: `X-Api-Key` header on all requests
- Endpoints used: `GET /tasks/search`, `POST /timers`, `DELETE /timers/current`
