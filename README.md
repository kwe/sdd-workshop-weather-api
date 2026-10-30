# SDD Workshop — Simple Weather API

A complete Rust starter project for an SDD workshop. It provides a small,
working Weather API that participants can run, explore, and extend using their
preferred Spec-Driven Development workflow.

## Tech Stack

- Rust (stable, 2024 edition)
- Axum for HTTP routing
- SQLx with SQLite for async database access
- Tokio runtime
- Serde for JSON
- utoipa and Swagger UI for OpenAPI documentation

## Prerequisites

- Rust stable toolchain (`rustc` and `cargo`)
- A C toolchain may be required to build SQLite dependencies on some platforms

## Build and Run

```bash
cargo run
```

The application starts on `http://localhost:8080` and creates `weather.db` in
the current directory. The first run applies the database migration and seeds
sample records automatically.

Useful commands:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Set `DATABASE_URL` to use a different SQLite file:

```bash
DATABASE_URL=sqlite://./weather.db cargo run
```

On PowerShell:

```powershell
$env:DATABASE_URL = "sqlite://./weather.db"
cargo run
```

## API Documentation

Swagger UI: `http://localhost:8080/swagger-ui.html`  
OpenAPI JSON: `http://localhost:8080/v3/api-docs`

### Endpoints

| Method | Endpoint | Description |
| --- | --- | --- |
| GET | `/api/weather` | List all weather records |
| GET | `/api/weather?city=London` | List weather records filtered by city, case-insensitively |
| GET | `/api/weather/{id}` | Get a single weather record by ID |
| POST | `/api/weather` | Create a new weather record |
| DELETE | `/api/weather/{id}` | Delete a weather record by ID |
| GET | `/api/weather/forecast/{city}` | 5-day forecast (today through today + 4, UTC dates), city case-insensitive |

The forecast data is seeded and static; only the dates roll forward. Unknown
cities return 404 and a blank city returns 400.

Example: `GET /api/weather/forecast/london`

```json
[{"date":"2026-10-30","highTemperatureCelsius":19.0,"lowTemperatureCelsius":11.0,"conditions":"Cloudy","precipitationChance":40}]
```
(truncated to the first of 5 days)

### Example POST Body

```json
{
  "city": "Madrid",
  "country": "Spain",
  "temperatureCelsius": 29.4,
  "humidity": 35,
  "description": "Sunny",
  "windSpeedKmh": 7.2,
  "recordedAt": "2026-07-17T10:00:00"
}
```

`city` is required. `temperatureCelsius`, when provided, must be between -90
and 60. `humidity`, when provided, must be between 0 and 100. If `recordedAt`
is omitted, the current date and time is used.

Validation and not-found responses use this shape:

```json
{
  "timestamp": "2026-07-17T08:15:30",
  "status": 400,
  "error": "Bad Request",
  "message": "City is required",
  "path": "/api/weather"
}
```

## Seed Data

The SQLite migration creates sample weather records for London, New York,
Tokyo, Sydney, Paris, Berlin, Mumbai, São Paulo, Cape Town, and Toronto. Migration `0002` seeds a 5-day forecast template for the same cities.

## Testing

The test suite covers HTTP behavior, validation, error responses, and the
SQLite-backed repository using an in-memory database:

```bash
cargo test
```

## Workshop Challenge

Using the SDD framework of your choice, plan and implement one of these
extensions:

1. Add weather forecasting (5-day forecast endpoint)
2. Add location-based search (latitude/longitude coordinates)
3. Add a weather alerts/notifications system
4. Add historical weather data and trends

## Project Structure

```text
weather-api/
├── migrations/
│   └── 0001_create_weather_records.sql
├── src/
│   ├── db.rs
│   ├── error.rs
│   ├── handlers.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models.rs
│   ├── repository.rs
│   └── service.rs
├── tests/
│   └── api.rs
├── Cargo.toml
└── README.md
```
