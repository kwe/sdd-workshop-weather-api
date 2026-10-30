use crate::{
    error::{ApiError, json_error, with_path},
    models::{ErrorResponse, WeatherRecord},
    repository,
};
use axum::{
    Json, Router,
    extract::{Path, State, rejection::JsonRejection},
    http::{StatusCode, Uri},
    routing::{delete, get},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;
use utoipa::ToSchema;

const MAX_CITY_CHARS: usize = 100;

#[derive(Debug, Clone, FromRow)]
struct FavouriteRow {
    city: String,
    city_key: String,
    created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Favourite {
    /// Canonical city name taken from the latest weather record
    pub city: String,
    /// When the city was starred (UTC, `YYYY-MM-DDTHH:MM:SS`)
    pub favourited_at: String,
    /// Latest weather record for the city, or null if none remain
    pub weather: Option<WeatherRecord>,
}

/// Accepts any JSON type so a non-string `city` yields 400 instead of 422.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CityInput {
    Text(String),
    Other(serde::de::IgnoredAny),
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFavouriteRequest {
    /// City name; trimmed, 1-100 characters, matched case-insensitively
    #[schema(value_type = Option<String>, example = "London")]
    pub city: Option<CityInput>,
}

fn normalize_city(city: &str) -> Result<(String, String), ApiError> {
    let display = city.trim();
    if display.is_empty() {
        return Err(ApiError::bad_request("City is required"));
    }
    if display.chars().count() > MAX_CITY_CHARS {
        return Err(ApiError::bad_request(format!(
            "City must be at most {MAX_CITY_CHARS} characters"
        )));
    }
    Ok((display.to_owned(), display.to_lowercase()))
}

/// Latest record = greatest recorded_at, ties broken by greatest id.
fn latest_by_city(records: Vec<WeatherRecord>) -> HashMap<String, WeatherRecord> {
    let mut latest: HashMap<String, WeatherRecord> = HashMap::new();
    for record in records {
        let key = record.city.to_lowercase();
        let newer = latest.get(&key).is_none_or(|current| {
            (&record.recorded_at, record.id) > (&current.recorded_at, current.id)
        });
        if newer {
            latest.insert(key, record);
        }
    }
    latest
}

fn to_favourite(row: FavouriteRow, latest: &HashMap<String, WeatherRecord>) -> Favourite {
    let weather = latest.get(&row.city_key).cloned();
    Favourite {
        city: weather
            .as_ref()
            .map_or(row.city, |record| record.city.clone()),
        favourited_at: row.created_at,
        weather,
    }
}

async fn favourite_row(pool: &SqlitePool, key: &str) -> Result<Option<FavouriteRow>, sqlx::Error> {
    sqlx::query_as::<_, FavouriteRow>(
        "SELECT city, city_key, created_at FROM favourites WHERE city_key = ?",
    )
    .bind(key)
    .fetch_optional(pool)
    .await
}

async fn list_favourites(pool: &SqlitePool) -> Result<Vec<Favourite>, ApiError> {
    let rows = sqlx::query_as::<_, FavouriteRow>(
        "SELECT city, city_key, created_at FROM favourites ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await?;
    let latest = latest_by_city(repository::list_weather_records(pool, None).await?);
    Ok(rows
        .into_iter()
        .map(|row| to_favourite(row, &latest))
        .collect())
}

async fn add_favourite(
    pool: &SqlitePool,
    city: Option<CityInput>,
) -> Result<(bool, Favourite), ApiError> {
    let Some(CityInput::Text(city)) = city else {
        return Err(ApiError::bad_request("City is required"));
    };
    let (display, key) = normalize_city(&city)?;
    let records = repository::list_weather_records(pool, Some(&display)).await?;
    let latest = latest_by_city(records);
    let Some(record) = latest.get(&key) else {
        return Err(ApiError::not_found(format!(
            "No weather records found for city {display}"
        )));
    };
    let created_at = Utc::now()
        .naive_utc()
        .format("%Y-%m-%dT%H:%M:%S%.f")
        .to_string();
    let created = sqlx::query(
        "INSERT INTO favourites (city, city_key, created_at) VALUES (?, ?, ?) \
         ON CONFLICT(city_key) DO NOTHING",
    )
    .bind(&record.city)
    .bind(&key)
    .bind(created_at)
    .execute(pool)
    .await?
    .rows_affected()
        > 0;
    let row = favourite_row(pool, &key)
        .await?
        .ok_or_else(ApiError::internal)?;
    Ok((created, to_favourite(row, &latest)))
}

async fn remove_favourite(pool: &SqlitePool, city: &str) -> Result<(), ApiError> {
    let (display, key) = normalize_city(city)?;
    let result = sqlx::query("DELETE FROM favourites WHERE city_key = ?")
        .bind(key)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::not_found(format!("{display} is not a favourite")));
    }
    Ok(())
}

/// Lists favourites, newest first, each with the city's latest weather record.
#[utoipa::path(
    get,
    path = "/api/favourites",
    tag = "favourites",
    responses(
        (status = 200, description = "Favourite cities, newest first", body = [Favourite]),
        (status = 500, description = "Database error", body = ErrorResponse)
    )
)]
pub async fn get_favourites(
    State(pool): State<SqlitePool>,
    uri: Uri,
) -> Result<Json<Vec<Favourite>>, ApiError> {
    list_favourites(&pool)
        .await
        .map(Json)
        .map_err(|error| with_path(error, &uri))
}

/// Stars a city. Idempotent: an existing favourite is returned with 200.
#[utoipa::path(
    post,
    path = "/api/favourites",
    tag = "favourites",
    request_body = CreateFavouriteRequest,
    responses(
        (status = 201, description = "Favourite created", body = Favourite),
        (status = 200, description = "City was already a favourite", body = Favourite),
        (status = 400, description = "City missing, blank, longer than 100 characters, or malformed body", body = ErrorResponse),
        (status = 404, description = "City has no weather records", body = ErrorResponse)
    )
)]
pub async fn create_favourite(
    State(pool): State<SqlitePool>,
    uri: Uri,
    payload: Result<Json<CreateFavouriteRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<Favourite>), ApiError> {
    let Json(request) = payload.map_err(|error| json_error(error, &uri))?;
    let (created, favourite) = add_favourite(&pool, request.city)
        .await
        .map_err(|error| with_path(error, &uri))?;
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(favourite)))
}

/// Removes a favourite (city matched case-insensitively).
#[utoipa::path(
    delete,
    path = "/api/favourites/{city}",
    tag = "favourites",
    params(("city" = String, Path, description = "City name (case-insensitive)")),
    responses(
        (status = 204, description = "Favourite removed"),
        (status = 400, description = "City is blank or too long", body = ErrorResponse),
        (status = 404, description = "City is not a favourite", body = ErrorResponse)
    )
)]
pub async fn delete_favourite(
    State(pool): State<SqlitePool>,
    Path(city): Path<String>,
    uri: Uri,
) -> Result<StatusCode, ApiError> {
    remove_favourite(&pool, &city)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|error| with_path(error, &uri))
}

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route(
            "/api/favourites",
            get(get_favourites).post(create_favourite),
        )
        .route("/api/favourites/{city}", delete(delete_favourite))
}
