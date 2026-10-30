use crate::{
    error::ApiError,
    models::{CreateWeatherRecordRequest, ForecastDay, ForecastRow, WeatherRecord},
    repository,
};
use chrono::{Days, NaiveDate, Utc};
use sqlx::SqlitePool;

pub async fn get_forecast(
    pool: &SqlitePool,
    city: &str,
    today: NaiveDate,
) -> Result<Vec<ForecastDay>, ApiError> {
    let (display, key) = normalize_city(city)?;
    let rows = repository::list_forecast_rows(pool, &key).await?;
    if rows.is_empty() {
        return Err(ApiError::not_found(format!(
            "No forecast found for city {display}"
        )));
    }
    Ok(rows
        .into_iter()
        .map(|row| to_forecast_day(row, today))
        .collect())
}

fn normalize_city(city: &str) -> Result<(String, String), ApiError> {
    let display = city.trim();
    if display.is_empty() {
        return Err(ApiError::bad_request("City is required"));
    }
    Ok((display.to_owned(), display.to_lowercase()))
}

fn to_forecast_day(row: ForecastRow, today: NaiveDate) -> ForecastDay {
    let date = today
        .checked_add_days(Days::new(row.day_offset.max(0) as u64))
        .unwrap_or(today);
    ForecastDay {
        date: date.format("%Y-%m-%d").to_string(),
        high_temperature_celsius: row.high_temperature_celsius,
        low_temperature_celsius: row.low_temperature_celsius,
        conditions: row.conditions,
        precipitation_chance: row.precipitation_chance,
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn row(day_offset: i32) -> ForecastRow {
        ForecastRow {
            day_offset,
            high_temperature_celsius: 20.5,
            low_temperature_celsius: 10.0,
            conditions: "Sunny".to_owned(),
            precipitation_chance: 15,
        }
    }

    #[test]
    fn maps_day_offsets_to_dates_across_a_month_boundary() {
        let today = NaiveDate::from_ymd_opt(2026, 10, 30).unwrap();
        assert_eq!(to_forecast_day(row(0), today).date, "2026-10-30");
        assert_eq!(to_forecast_day(row(4), today).date, "2026-11-03");
    }

    #[test]
    fn copies_forecast_values() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let day = to_forecast_day(row(1), today);
        assert_eq!(day.high_temperature_celsius, 20.5);
        assert_eq!(day.low_temperature_celsius, 10.0);
        assert_eq!(day.conditions, "Sunny");
        assert_eq!(day.precipitation_chance, 15);
    }

    #[test]
    fn normalizes_city_with_trim_and_unicode_lowercase() {
        let (display, key) = normalize_city("  São Paulo ").unwrap();
        assert_eq!(display, "São Paulo");
        assert_eq!(key, "são paulo");
    }

    #[test]
    fn rejects_blank_city() {
        assert!(normalize_city("   ").is_err());
    }
}
