use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeatherRecord {
    pub id: i64,
    pub city: String,
    pub country: Option<String>,
    pub temperature_celsius: Option<f64>,
    pub humidity: Option<i32>,
    pub description: Option<String>,
    pub wind_speed_kmh: Option<f64>,
    pub recorded_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWeatherRecordRequest {
    pub city: Option<String>,
    pub country: Option<String>,
    pub temperature_celsius: Option<f64>,
    pub humidity: Option<i32>,
    pub description: Option<String>,
    pub wind_speed_kmh: Option<f64>,
    pub recorded_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ForecastRow {
    pub day_offset: i32,
    pub high_temperature_celsius: f64,
    pub low_temperature_celsius: f64,
    pub conditions: String,
    pub precipitation_chance: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ForecastDay {
    /// Calendar date (UTC) in `YYYY-MM-DD` format
    pub date: String,
    pub high_temperature_celsius: f64,
    pub low_temperature_celsius: f64,
    pub conditions: String,
    /// Chance of precipitation as a percentage (0-100)
    pub precipitation_chance: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub timestamp: NaiveDateTime,
    pub status: u16,
    pub error: String,
    pub message: String,
    pub path: String,
}
