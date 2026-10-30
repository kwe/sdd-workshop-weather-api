use crate::{
    error::ApiError,
    models::{CreateWeatherRecordRequest, WeatherRecord},
    repository,
};
use chrono::Utc;
use sqlx::SqlitePool;

pub async fn list_weather_records(
    pool: &SqlitePool,
    city: Option<&str>,
) -> Result<Vec<WeatherRecord>, ApiError> {
    let city = city.map(str::trim).filter(|city| !city.is_empty());
    Ok(repository::list_weather_records(pool, city).await?)
}

pub async fn get_weather_record(pool: &SqlitePool, id: i64) -> Result<WeatherRecord, ApiError> {
    repository::get_weather_record(pool, id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Weather record not found for id {id}")))
}

pub async fn create_weather_record(
    pool: &SqlitePool,
    request: CreateWeatherRecordRequest,
) -> Result<WeatherRecord, ApiError> {
    let city = request
        .city
        .filter(|city| !city.trim().is_empty())
        .ok_or_else(|| ApiError::bad_request("City is required"))?;

    if let Some(temperature) = request.temperature_celsius
        && !(-90.0..=60.0).contains(&temperature)
    {
        return Err(ApiError::bad_request(
            "Temperature must be between -90 and 60 Celsius",
        ));
    }

    if let Some(humidity) = request.humidity
        && !(0..=100).contains(&humidity)
    {
        return Err(ApiError::bad_request("Humidity must be between 0 and 100"));
    }

    let recorded_at = request
        .recorded_at
        .unwrap_or_else(|| Utc::now().naive_utc())
        .format("%Y-%m-%dT%H:%M:%S%.f")
        .to_string();
    let record = WeatherRecord {
        id: 0,
        city: city.trim().to_owned(),
        country: request.country,
        temperature_celsius: request.temperature_celsius,
        humidity: request.humidity,
        description: request.description,
        wind_speed_kmh: request.wind_speed_kmh,
        recorded_at,
    };
    let id = repository::insert_weather_record(pool, &record).await?;

    Ok(WeatherRecord { id, ..record })
}

pub async fn delete_weather_record(pool: &SqlitePool, id: i64) -> Result<(), ApiError> {
    if repository::delete_weather_record(pool, id).await? {
        Ok(())
    } else {
        Err(ApiError::not_found(format!(
            "Weather record not found for id {id}"
        )))
    }
}
