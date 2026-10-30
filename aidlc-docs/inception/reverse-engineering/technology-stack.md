# Technology Stack

## Programming Languages
- Rust (stable, edition 2024) - all application code

## Frameworks
- Axum 0.8 - HTTP routing
- SQLx 0.8 - async DB access and migrations
- utoipa 5 / utoipa-swagger-ui 9 - OpenAPI

## Infrastructure
- SQLite file `weather.db` (local); no deployment infra defined

## Build Tools
- Cargo; `cargo fmt`, `cargo clippy -D warnings` documented in README

## Testing Tools
- Tokio async tests, tower `ServiceExt::oneshot`, http-body-util, serde_json
