# Build Instructions

## Prerequisites
- **Build tool**: Rust toolchain with Cargo (edition 2024, Rust 1.85+)
- **Dependencies**: fetched by Cargo (axum, sqlx/sqlite, tokio, chrono, utoipa)
- **Environment variables**: `DATABASE_URL` (optional, default `sqlite://weather.db`)
- **System**: any OS with a C toolchain (SQLite is bundled via sqlx)

## Build Steps
1. Dependencies: resolved automatically by `cargo build`.
2. Configure: none required. Migrations (`0001`, `0002`) run at startup.
3. Build:
   ```bash
   cargo build --release
   ```
4. Verify:
   - Output ends with `Finished release profile`; binary at `target/release/sdd-workshop-weather-api`.
   - Quality gates:
     ```bash
     cargo fmt --check
     cargo clippy --all-targets --all-features -- -D warnings
     ```
   - No warnings expected.

## Run
```bash
DATABASE_URL="sqlite://weather.db?mode=rwc" ./target/release/sdd-workshop-weather-api
```
Listens on port 8080 (fixed in `src/main.rs`).

## Troubleshooting
- **Dependency errors**: check network access, then retry `cargo fetch`.
- **Compilation errors**: confirm `rustc --version` is 1.85+ (edition 2024).
- **Port in use**: stop the other process on 8080.
