# Services

## ForecastService (functions in service.rs)
- **Responsibility**: Orchestrates a forecast request: validate and normalise the city, fetch rows from the repository, translate "no rows" into a 404, and map rows to dated DTOs.
- **Entry point**: `service::get_forecast(pool, city, today)`.
- **Orchestration**:
  1. `normalize_city(city)` -> blank returns `ApiError::bad_request("City is required")`
  2. `repository::list_forecast_rows(pool, key)`
  3. Empty -> `ApiError::not_found("No forecast found for city {display}")`
  4. Map each row via `to_forecast_day(row, today)`
- **Error handling**: `sqlx::Error` -> 500 via existing `From`; handler attaches the path via `with_path`.
- **Determinism**: `today` is passed in by the handler (Q2 A), so date logic is unit-testable without a clock.

## Existing services
Weather record functions are unchanged.
