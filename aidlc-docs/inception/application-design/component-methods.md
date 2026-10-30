# Component Methods (signatures only)

Detailed business rules are specified in Functional Design.

## models.rs
```rust
pub struct ForecastDay { /* date, high/low celsius, conditions, precipitation_chance */ }
pub struct ForecastRow { /* day_offset, high/low celsius, conditions, precipitation_chance */ }
```

## repository.rs
```rust
pub async fn list_forecast_rows(
    pool: &SqlitePool,
    city_key: &str,
) -> Result<Vec<ForecastRow>, sqlx::Error>;
```
Purpose: single indexed equality query by `city_key`, ordered by `day_offset`.

## service.rs
```rust
pub async fn get_forecast(
    pool: &SqlitePool,
    city: &str,
    today: NaiveDate,
) -> Result<Vec<ForecastDay>, ApiError>;

fn normalize_city(city: &str) -> Result<(String /*display*/, String /*key*/), ApiError>; // private
fn to_forecast_day(row: ForecastRow, today: NaiveDate) -> ForecastDay;                   // private/pure
```
Purpose: validate (blank -> 400), normalise (trim + `to_lowercase`), 404 if empty, map rows with `date = today + day_offset` (`YYYY-MM-DD`).

## handlers.rs
```rust
pub async fn get_forecast(
    State(pool): State<SqlitePool>,
    Path(city): Path<String>,
    uri: Uri,
) -> Result<Json<Vec<ForecastDay>>, ApiError>;
```
Documented responses: 200 `[ForecastDay]`, 400 and 404 `ErrorResponse`, 500.

## lib.rs
```rust
.route("/api/weather/forecast/{city}", get(handlers::get_forecast))
// ApiDoc: paths(+ handlers::get_forecast), schemas(+ models::ForecastDay)
```
