use crate::{
    error::{ApiError, bad_path_id, json_error, with_path},
    models::{CreateWeatherRecordRequest, ErrorResponse, ForecastDay, WeatherRecord},
    service,
};
use axum::{
    Json,
    extract::{Path, Query, State, rejection::JsonRejection},
    http::{StatusCode, Uri},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct WeatherQuery {
    city: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/weather",
    params(WeatherQuery),
    responses(
        (status = 200, description = "Weather records", body = [WeatherRecord]),
        (status = 500, description = "Database error", body = ErrorResponse)
    )
)]
pub async fn get_weather_records(
    State(pool): State<SqlitePool>,
    Query(query): Query<WeatherQuery>,
) -> Result<Json<Vec<WeatherRecord>>, ApiError> {
    service::list_weather_records(&pool, query.city.as_deref())
        .await
        .map(Json)
        .map_err(|error| with_path(error, &Uri::from_static("/api/weather")))
}

#[utoipa::path(
    get,
    path = "/api/weather/{id}",
    params(("id" = i64, Path, description = "Weather record ID")),
    responses(
        (status = 200, description = "Weather record", body = WeatherRecord),
        (status = 400, description = "Invalid ID", body = ErrorResponse),
        (status = 404, description = "Weather record not found", body = ErrorResponse)
    )
)]
pub async fn get_weather_record(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    uri: Uri,
) -> Result<Json<WeatherRecord>, ApiError> {
    let id = id.parse::<i64>().ok().ok_or_else(|| bad_path_id(&uri))?;
    service::get_weather_record(&pool, id)
        .await
        .map(Json)
        .map_err(|error| with_path(error, &uri))
}

/// Returns a 5-day forecast (today through today + 4, UTC dates). Data is seeded and static.
#[utoipa::path(
    get,
    path = "/api/weather/forecast/{city}",
    params(("city" = String, Path, description = "City name (case-insensitive)")),
    responses(
        (status = 200, description = "Five daily forecasts ordered by date", body = [ForecastDay]),
        (status = 400, description = "City is blank", body = ErrorResponse),
        (status = 404, description = "No forecast found for city", body = ErrorResponse),
        (status = 500, description = "Forecast data incomplete or database error", body = ErrorResponse)
    )
)]
pub async fn get_forecast(
    State(pool): State<SqlitePool>,
    Path(city): Path<String>,
    uri: Uri,
) -> Result<Json<Vec<ForecastDay>>, ApiError> {
    service::get_forecast(&pool, &city, Utc::now().date_naive())
        .await
        .map(Json)
        .map_err(|error| with_path(error, &uri))
}

#[utoipa::path(
    post,
    path = "/api/weather",
    request_body = CreateWeatherRecordRequest,
    responses(
        (status = 201, description = "Weather record created", body = WeatherRecord),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    )
)]
pub async fn create_weather_record(
    State(pool): State<SqlitePool>,
    uri: Uri,
    payload: Result<Json<CreateWeatherRecordRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<WeatherRecord>), ApiError> {
    let Json(request) = payload.map_err(|error| json_error(error, &uri))?;
    let record = service::create_weather_record(&pool, request)
        .await
        .map_err(|error: ApiError| with_path(error, &uri))?;
    Ok((StatusCode::CREATED, Json(record)))
}

#[utoipa::path(
    delete,
    path = "/api/weather/{id}",
    params(("id" = i64, Path, description = "Weather record ID")),
    responses(
        (status = 204, description = "Weather record deleted"),
        (status = 400, description = "Invalid ID", body = ErrorResponse),
        (status = 404, description = "Weather record not found", body = ErrorResponse)
    )
)]
pub async fn delete_weather_record(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    uri: Uri,
) -> Result<StatusCode, ApiError> {
    let id = id.parse::<i64>().ok().ok_or_else(|| bad_path_id(&uri))?;
    service::delete_weather_record(&pool, id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|error| with_path(error, &uri))
}
