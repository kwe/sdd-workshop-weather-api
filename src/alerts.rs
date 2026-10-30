use crate::{
    error::{ApiError, with_path},
    models::{ErrorResponse, WeatherRecord},
    repository,
};
use axum::{Json, extract::State, http::Uri};
use chrono::{Days, NaiveDate, Utc};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use std::{cmp::Reverse, collections::HashMap};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AlertType {
    Heat,
    Cold,
    Wind,
    Humidity,
}

impl AlertType {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "heat" => Some(Self::Heat),
            "cold" => Some(Self::Cold),
            "wind" => Some(Self::Wind),
            "humidity" => Some(Self::Humidity),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Watch,
    Warning,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AlertSource {
    Current,
    Forecast,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub city: String,
    pub country: Option<String>,
    /// Kind of dangerous weather
    #[serde(rename = "type")]
    pub alert_type: AlertType,
    pub severity: Severity,
    /// Measured (current) or forecast value that breached the threshold
    pub value: f64,
    /// Configured base (`watch`) threshold for this alert type
    pub threshold: f64,
    pub source: AlertSource,
    /// `YYYY-MM-DD`: record date for current alerts, forecast day (UTC) for forecast alerts
    pub date: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Threshold {
    pub alert_type: String,
    pub comparison: String,
    pub threshold: f64,
    pub warning_threshold: f64,
    pub severe_threshold: f64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ForecastEntry {
    pub city_key: String,
    pub day_offset: i32,
    pub high_temperature_celsius: f64,
    pub low_temperature_celsius: f64,
}

fn classify(threshold: &Threshold, value: f64) -> Option<Severity> {
    let reached = |cut_off: f64| match threshold.comparison.as_str() {
        "gte" => value >= cut_off,
        "lte" => value <= cut_off,
        _ => false,
    };
    if reached(threshold.severe_threshold) {
        Some(Severity::Severe)
    } else if reached(threshold.warning_threshold) {
        Some(Severity::Warning)
    } else if reached(threshold.threshold) {
        Some(Severity::Watch)
    } else {
        None
    }
}

fn city_key(city: &str) -> String {
    city.trim().to_lowercase()
}

/// Latest record per city: greatest `recorded_at`, ties broken by greatest id.
fn latest_per_city(records: &[WeatherRecord]) -> Vec<&WeatherRecord> {
    let mut latest: HashMap<String, &WeatherRecord> = HashMap::new();
    for record in records {
        let key = city_key(&record.city);
        match latest.get(&key) {
            Some(prev) if (&prev.recorded_at, prev.id) >= (&record.recorded_at, record.id) => {}
            _ => {
                latest.insert(key, record);
            }
        }
    }
    latest.into_values().collect()
}

/// Pure evaluation of every city's latest record and forecast against the thresholds.
pub fn evaluate(
    thresholds: &[Threshold],
    records: &[WeatherRecord],
    forecasts: &[ForecastEntry],
    today: NaiveDate,
) -> Vec<Alert> {
    let rules: Vec<(AlertType, &Threshold)> = thresholds
        .iter()
        .filter_map(|t| AlertType::parse(&t.alert_type).map(|kind| (kind, t)))
        .collect();
    let mut alerts = Vec::new();

    for record in latest_per_city(records) {
        let make = |kind: AlertType,
                    t: &Threshold,
                    severity: Severity,
                    value: f64,
                    source: AlertSource,
                    date: String| Alert {
            city: record.city.clone(),
            country: record.country.clone(),
            alert_type: kind,
            severity,
            value,
            threshold: t.threshold,
            source,
            date,
        };

        let current_date: String = record.recorded_at.chars().take(10).collect();
        for (kind, t) in &rules {
            let value = match kind {
                AlertType::Heat | AlertType::Cold => record.temperature_celsius,
                AlertType::Wind => record.wind_speed_kmh,
                AlertType::Humidity => record.humidity.map(f64::from),
            };
            if let Some(value) = value
                && let Some(severity) = classify(t, value)
            {
                alerts.push(make(
                    *kind,
                    t,
                    severity,
                    value,
                    AlertSource::Current,
                    current_date.clone(),
                ));
            }
        }

        let key = city_key(&record.city);
        for day in forecasts.iter().filter(|day| day.city_key == key) {
            let date = today
                .checked_add_days(Days::new(day.day_offset.max(0) as u64))
                .unwrap_or(today)
                .format("%Y-%m-%d")
                .to_string();
            for (kind, t) in &rules {
                let value = match kind {
                    AlertType::Heat => day.high_temperature_celsius,
                    AlertType::Cold => day.low_temperature_celsius,
                    AlertType::Wind | AlertType::Humidity => continue,
                };
                if let Some(severity) = classify(t, value) {
                    alerts.push(make(
                        *kind,
                        t,
                        severity,
                        value,
                        AlertSource::Forecast,
                        date.clone(),
                    ));
                }
            }
        }
    }

    alerts.sort_by_cached_key(|a| {
        (
            Reverse(a.severity),
            a.city.to_lowercase(),
            a.date.clone(),
            a.source,
            a.alert_type,
        )
    });
    alerts
}

async fn list_thresholds(pool: &SqlitePool) -> Result<Vec<Threshold>, sqlx::Error> {
    sqlx::query_as::<_, Threshold>(
        "SELECT alert_type, comparison, threshold, warning_threshold, severe_threshold \
         FROM alert_thresholds WHERE enabled = 1",
    )
    .fetch_all(pool)
    .await
}

async fn list_forecast_entries(pool: &SqlitePool) -> Result<Vec<ForecastEntry>, sqlx::Error> {
    sqlx::query_as::<_, ForecastEntry>(
        "SELECT city_key, day_offset, high_temperature_celsius, low_temperature_celsius \
         FROM forecasts ORDER BY city_key, day_offset",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_alerts(pool: &SqlitePool, today: NaiveDate) -> Result<Vec<Alert>, ApiError> {
    let thresholds = list_thresholds(pool).await?;
    let records = repository::list_weather_records(pool, None).await?;
    let forecasts = list_forecast_entries(pool).await?;
    Ok(evaluate(&thresholds, &records, &forecasts, today))
}

/// Evaluates each city's latest weather record and 5-day forecast against the stored thresholds.
#[utoipa::path(
    get,
    path = "/api/alerts",
    tag = "alerts",
    responses(
        (status = 200, description = "Active weather alerts, most severe first", body = [Alert]),
        (status = 500, description = "Database error", body = ErrorResponse)
    )
)]
pub async fn get_alerts_handler(
    State(pool): State<SqlitePool>,
    uri: Uri,
) -> Result<Json<Vec<Alert>>, ApiError> {
    get_alerts(&pool, Utc::now().date_naive())
        .await
        .map(Json)
        .map_err(|error| with_path(error, &uri))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thr(kind: &str, comparison: &str, base: f64, warning: f64, severe: f64) -> Threshold {
        Threshold {
            alert_type: kind.to_owned(),
            comparison: comparison.to_owned(),
            threshold: base,
            warning_threshold: warning,
            severe_threshold: severe,
        }
    }

    fn defaults() -> Vec<Threshold> {
        vec![
            thr("heat", "gte", 30.0, 35.0, 40.0),
            thr("cold", "lte", 0.0, -5.0, -15.0),
            thr("wind", "gte", 40.0, 60.0, 90.0),
            thr("humidity", "gte", 85.0, 90.0, 95.0),
        ]
    }

    fn record(id: i64, city: &str, at: &str) -> WeatherRecord {
        WeatherRecord {
            id,
            city: city.to_owned(),
            country: Some("X".to_owned()),
            temperature_celsius: Some(20.0),
            humidity: Some(50),
            description: None,
            wind_speed_kmh: Some(10.0),
            recorded_at: at.to_owned(),
        }
    }

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, 30).unwrap()
    }

    fn eval(records: &[WeatherRecord], forecasts: &[ForecastEntry]) -> Vec<Alert> {
        evaluate(&defaults(), records, forecasts, today())
    }

    fn with_temp(t: f64) -> WeatherRecord {
        WeatherRecord {
            temperature_celsius: Some(t),
            ..record(1, "A", "2026-10-30T08:00:00")
        }
    }

    #[test]
    fn heat_boundary_is_inclusive() {
        assert!(eval(&[with_temp(29.9)], &[]).is_empty());
        let alerts = eval(&[with_temp(30.0)], &[]);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, AlertType::Heat);
        assert_eq!(alerts[0].severity, Severity::Watch);
        assert_eq!(alerts[0].threshold, 30.0);
        assert_eq!(alerts[0].date, "2026-10-30");
    }

    #[test]
    fn cold_boundary_is_inclusive_and_tiers_apply() {
        assert!(eval(&[with_temp(0.1)], &[]).is_empty());
        assert_eq!(eval(&[with_temp(0.0)], &[])[0].severity, Severity::Watch);
        assert_eq!(eval(&[with_temp(-5.0)], &[])[0].severity, Severity::Warning);
        let severe = eval(&[with_temp(-15.0)], &[]);
        assert_eq!(severe[0].severity, Severity::Severe);
        assert_eq!(severe[0].alert_type, AlertType::Cold);
    }

    #[test]
    fn heat_tiers_apply() {
        assert_eq!(eval(&[with_temp(34.9)], &[])[0].severity, Severity::Watch);
        assert_eq!(eval(&[with_temp(35.0)], &[])[0].severity, Severity::Warning);
        assert_eq!(eval(&[with_temp(40.0)], &[])[0].severity, Severity::Severe);
    }

    #[test]
    fn wind_and_humidity_boundaries() {
        let r = WeatherRecord {
            wind_speed_kmh: Some(40.0),
            humidity: Some(85),
            ..record(1, "A", "2026-10-30T08:00:00")
        };
        let alerts = eval(&[r], &[]);
        let kinds: Vec<_> = alerts.iter().map(|a| a.alert_type).collect();
        assert_eq!(kinds, vec![AlertType::Wind, AlertType::Humidity]);
        let r = WeatherRecord {
            wind_speed_kmh: Some(39.9),
            humidity: Some(84),
            ..record(1, "A", "2026-10-30T08:00:00")
        };
        assert!(eval(&[r], &[]).is_empty());
    }

    #[test]
    fn null_metrics_produce_no_alerts() {
        let r = WeatherRecord {
            temperature_celsius: None,
            humidity: None,
            wind_speed_kmh: None,
            ..record(1, "A", "2026-10-30T08:00:00")
        };
        assert!(eval(&[r], &[]).is_empty());
    }

    #[test]
    fn disabled_or_unknown_thresholds_are_ignored() {
        let thresholds = vec![thr("tornado", "gte", 1.0, 2.0, 3.0)];
        assert!(evaluate(&thresholds, &[with_temp(50.0)], &[], today()).is_empty());
        assert!(evaluate(&[], &[with_temp(50.0)], &[], today()).is_empty());
    }

    fn forecast(city_key: &str, day_offset: i32, high: f64, low: f64) -> ForecastEntry {
        ForecastEntry {
            city_key: city_key.to_owned(),
            day_offset,
            high_temperature_celsius: high,
            low_temperature_celsius: low,
        }
    }

    #[test]
    fn forecast_alerts_use_high_for_heat_low_for_cold_and_map_dates() {
        let forecasts = vec![
            forecast("a", 0, 25.0, 10.0),
            forecast("a", 1, 31.0, 10.0),
            forecast("a", 3, 25.0, -1.0),
            forecast("other", 2, 45.0, -20.0),
        ];
        let alerts = eval(&[record(1, "A", "2026-10-30T08:00:00")], &forecasts);
        assert_eq!(alerts.len(), 2);
        assert!(alerts.iter().all(|a| a.source == AlertSource::Forecast));
        let heat = alerts
            .iter()
            .find(|a| a.alert_type == AlertType::Heat)
            .unwrap();
        assert_eq!((heat.value, heat.date.as_str()), (31.0, "2026-10-31"));
        let cold = alerts
            .iter()
            .find(|a| a.alert_type == AlertType::Cold)
            .unwrap();
        assert_eq!((cold.value, cold.date.as_str()), (-1.0, "2026-11-02"));
    }

    #[test]
    fn forecast_matching_is_case_insensitive_with_unicode() {
        let forecasts = vec![forecast("são paulo", 0, 33.0, 20.0)];
        let alerts = eval(
            &[record(1, "  São Paulo ", "2026-10-30T08:00:00")],
            &forecasts,
        );
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].city, "  São Paulo ");
    }

    #[test]
    fn city_without_forecast_still_gets_current_alerts() {
        let alerts = eval(&[with_temp(36.0)], &[]);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].source, AlertSource::Current);
    }

    #[test]
    fn only_latest_record_per_city_is_evaluated() {
        let hot_old = WeatherRecord {
            temperature_celsius: Some(41.0),
            ..record(1, "A", "2026-10-29T08:00:00")
        };
        let mild_new = record(2, "a", "2026-10-30T08:00:00");
        assert!(eval(&[hot_old.clone(), mild_new], &[]).is_empty());

        let same_time_hot = WeatherRecord { id: 3, ..hot_old };
        let tie_old = record(2, "A", "2026-10-29T08:00:00");
        assert_eq!(eval(&[tie_old, same_time_hot], &[]).len(), 1);
    }

    #[test]
    fn alerts_are_sorted_by_severity_then_city_then_date_then_source() {
        let hot = |id, city: &str, t| WeatherRecord {
            temperature_celsius: Some(t),
            ..record(id, city, "2026-10-30T08:00:00")
        };
        let forecasts = vec![forecast("beta", 1, 31.0, 20.0)];
        let alerts = eval(
            &[
                hot(1, "beta", 31.0),
                hot(2, "Alpha", 31.0),
                hot(3, "Zed", 41.0),
            ],
            &forecasts,
        );
        let order: Vec<_> = alerts
            .iter()
            .map(|a| (a.city.as_str(), a.severity, a.source))
            .collect();
        assert_eq!(
            order,
            vec![
                ("Zed", Severity::Severe, AlertSource::Current),
                ("Alpha", Severity::Watch, AlertSource::Current),
                ("beta", Severity::Watch, AlertSource::Current),
                ("beta", Severity::Watch, AlertSource::Forecast),
            ]
        );
    }
}
