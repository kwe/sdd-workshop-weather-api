# Application Design (Consolidated, Minimal Depth)

**Feature**: `GET /api/weather/forecast/{city}` | **Stories**: S1-S5 | **Design decisions**: Q1 A (existing files), Q2 A (`today` parameter), Q3 A (`ForecastRow` -> `ForecastDay`), Q4 B (`city_key` column, hand-lowercased seed, index, equality query; service normalises via trim + `to_lowercase()`), Q5 A (`Path<String>` + `Uri`; service validates).

## Components
| Component | Change |
|---|---|
| models.rs | add `ForecastDay` (public/OpenAPI), `ForecastRow` (internal) |
| repository.rs | add `list_forecast_rows(pool, city_key)` |
| service.rs | add `get_forecast(pool, city, today)` + private `normalize_city`, `to_forecast_day` |
| handlers.rs | add `get_forecast` with utoipa path |
| lib.rs | add GET route and OpenAPI registrations |
| migrations/0002_create_forecasts.sql | table, CHECKs, unique (`city_key`, `day_offset`), index on `city_key`, seed 10 cities x 5 days |
| error.rs | unchanged |

## Signatures
```rust
repository::list_forecast_rows(pool: &SqlitePool, city_key: &str) -> Result<Vec<ForecastRow>, sqlx::Error>
service::get_forecast(pool: &SqlitePool, city: &str, today: NaiveDate) -> Result<Vec<ForecastDay>, ApiError>
handlers::get_forecast(State<SqlitePool>, Path<String>, Uri) -> Result<Json<Vec<ForecastDay>>, ApiError>
```

## Service Flow
handler (today = UTC date) -> service (trim, blank 400, lowercase) -> repository (indexed equality query ordered by `day_offset`) -> service (empty 404, map to `date = today + offset`) -> JSON array.

## Dependencies
handlers -> service -> repository -> SQLite; no new crates; independent of `weather_records`.

## Coverage Check vs Stories
| Story | Design element |
|---|---|
| S1 | repository query, `to_forecast_day`, seed of 10 cities x 5 days |
| S2 | `normalize_city` (trim, Unicode `to_lowercase`), axum percent-decoding, `city_key` |
| S3 | `ApiError::not_found` message, `with_path` |
| S4 | GET-only route (405 on other verbs), shared error type, no auth |
| S5 | migration CHECKs, unit tests (service) + HTTP tests, OpenAPI registration, README |

## Notes for Functional Design
- Seed `city_key` values must equal Rust `to_lowercase()` of the display city (e.g. `são paulo`); a test should assert this for all 10 cities.
- Exact seed values, conditions vocabulary and error message wording are finalised in Functional Design.
