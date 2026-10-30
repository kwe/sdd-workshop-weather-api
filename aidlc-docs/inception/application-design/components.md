# Components (Minimal Depth)

All additions go into existing files (Q1 A). No new modules or crates.

## C1 — models.rs
- **Purpose**: Public DTOs and schemas.
- **Additions**: `ForecastDay` (public, `Serialize`, `ToSchema`, camelCase): `date: String`, `high_temperature_celsius: f64`, `low_temperature_celsius: f64`, `conditions: String`, `precipitation_chance: i32`.
- **Additions (internal)**: `ForecastRow` (`FromRow`): `day_offset: i32`, `high_temperature_celsius: f64`, `low_temperature_celsius: f64`, `conditions: String`, `precipitation_chance: i32`.

## C2 — repository.rs
- **Purpose**: SQL access.
- **Addition**: query `forecasts` by normalised `city_key`, ordered by `day_offset`.

## C3 — service.rs
- **Purpose**: Business rules.
- **Additions**: city trim/validate (400), normalisation (`to_lowercase`), 404 when no rows, mapping `ForecastRow` -> `ForecastDay` with `date = today + day_offset`.

## C4 — handlers.rs
- **Purpose**: HTTP edge.
- **Addition**: `get_forecast` handler with `#[utoipa::path]` for `GET /api/weather/forecast/{city}`; extracts `Path<String>` and `Uri`, passes `Utc::now().date_naive()` to the service.

## C5 — lib.rs
- **Purpose**: Wiring.
- **Additions**: route `/api/weather/forecast/{city}` (GET only), register path and `ForecastDay` schema in `ApiDoc`.

## C6 — migrations/0002_create_forecasts.sql (new)
- **Purpose**: `forecasts` table, index, seed rows.
- **Columns**: `id`, `city`, `city_key` (lowercase), `day_offset`, `high_temperature_celsius`, `low_temperature_celsius`, `conditions`, `precipitation_chance`; CHECKs for high >= low, chance 0-100, offset 0-4; unique (`city_key`, `day_offset`); index on `city_key`.

## C7 — error.rs (unchanged)
Reuses `ApiError::bad_request`, `ApiError::not_found`, `with_path`.

## Test components
- `tests/api.rs`: HTTP tests for S1-S5. Unit tests in `service.rs` (`#[cfg(test)]`) for date mapping and normalisation.
