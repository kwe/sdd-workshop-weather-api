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

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub timestamp: NaiveDateTime,
    pub status: u16,
    pub error: String,
    pub message: String,
    pub path: String,
}
