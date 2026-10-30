use crate::models::{ForecastRow, WeatherRecord};
use sqlx::SqlitePool;

pub async fn list_forecast_rows(
    pool: &SqlitePool,
    city_key: &str,
) -> Result<Vec<ForecastRow>, sqlx::Error> {
    sqlx::query_as::<_, ForecastRow>(
        "SELECT day_offset, high_temperature_celsius, low_temperature_celsius, \
         conditions, precipitation_chance FROM forecasts \
         WHERE city_key = ? ORDER BY day_offset",
    )
    .bind(city_key)
    .fetch_all(pool)
    .await
}

pub async fn list_weather_records(
    pool: &SqlitePool,
    city: Option<&str>,
) -> Result<Vec<WeatherRecord>, sqlx::Error> {
    let records = sqlx::query_as::<_, WeatherRecord>(
        "SELECT id, city, country, temperature_celsius, humidity, description, \
         wind_speed_kmh, recorded_at FROM weather_records ORDER BY id",
    )
    .fetch_all(pool)
    .await?;

    match city {
        // SQLite's built-in lower() is ASCII-only; Rust handles Unicode city names here.
        Some(city) => Ok(records
            .into_iter()
            .filter(|record| record.city.to_lowercase() == city.to_lowercase())
            .collect()),
        None => Ok(records),
    }
}

pub async fn get_weather_record(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<WeatherRecord>, sqlx::Error> {
    sqlx::query_as::<_, WeatherRecord>(
        "SELECT id, city, country, temperature_celsius, humidity, description, \
         wind_speed_kmh, recorded_at FROM weather_records WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn insert_weather_record(
    pool: &SqlitePool,
    record: &WeatherRecord,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "INSERT INTO weather_records \
         (city, country, temperature_celsius, humidity, description, wind_speed_kmh, recorded_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id",
    )
    .bind(&record.city)
    .bind(&record.country)
    .bind(record.temperature_celsius)
    .bind(record.humidity)
    .bind(&record.description)
    .bind(record.wind_speed_kmh)
    .bind(&record.recorded_at)
    .fetch_one(pool)
    .await
}

pub async fn delete_weather_record(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM weather_records WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
