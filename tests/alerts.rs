use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use chrono::{Days, Utc};
use http_body_util::BodyExt;
use sdd_workshop_weather_api::{app, db};
use serde_json::{Value, json};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tower::ServiceExt;

async fn test_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite database should open");
    db::migrate(&pool)
        .await
        .expect("database migrations should succeed");
    pool
}

async fn request(pool: &SqlitePool, method: &str, uri: &str, body: Option<Value>) -> Response {
    let mut builder = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(body) => {
            builder = builder.header("content-type", "application/json");
            Body::from(body.to_string())
        }
        None => Body::empty(),
    };
    app(pool.clone())
        .oneshot(builder.body(body).expect("request should be valid"))
        .await
        .expect("router should handle the request")
}

async fn json_body(response: Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).expect("response should contain JSON")
}

async fn alerts(pool: &SqlitePool) -> Vec<Value> {
    let response = request(pool, "GET", "/api/alerts", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    json_body(response).await.as_array().unwrap().clone()
}

async fn post_weather(pool: &SqlitePool, body: Value) {
    let response = request(pool, "POST", "/api/weather", Some(body)).await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

async fn empty_pool() -> SqlitePool {
    let pool = test_pool().await;
    sqlx::query("DELETE FROM weather_records")
        .execute(&pool)
        .await
        .unwrap();
    pool
}

fn of_city<'a>(alerts: &'a [Value], city: &str) -> Vec<&'a Value> {
    alerts.iter().filter(|a| a["city"] == city).collect()
}

fn date_in(days: u64) -> String {
    Utc::now()
        .date_naive()
        .checked_add_days(Days::new(days))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string()
}

#[tokio::test]
async fn migration_seeds_default_thresholds() {
    let pool = test_pool().await;
    let rows: Vec<(String, String, f64, i64)> = sqlx::query_as(
        "SELECT alert_type, comparison, threshold, enabled FROM alert_thresholds ORDER BY alert_type",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows,
        vec![
            ("cold".to_owned(), "lte".to_owned(), 0.0, 1),
            ("heat".to_owned(), "gte".to_owned(), 30.0, 1),
            ("humidity".to_owned(), "gte".to_owned(), 85.0, 1),
            ("wind".to_owned(), "gte".to_owned(), 40.0, 1),
        ]
    );
}

#[tokio::test]
async fn threshold_table_rejects_inconsistent_tiers() {
    let pool = test_pool().await;
    let result =
        sqlx::query("UPDATE alert_thresholds SET severe_threshold = 10 WHERE alert_type = 'heat'")
            .execute(&pool)
            .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn seed_data_produces_expected_heat_alerts() {
    let pool = test_pool().await;
    let alerts = alerts(&pool).await;
    assert_eq!(alerts.len(), 10);
    assert!(alerts.iter().all(|a| a["type"] == "heat"));
    assert!(alerts.iter().all(|a| a["severity"] == "watch"));
    assert!(alerts.iter().all(|a| a["threshold"] == 30.0));

    let mumbai = of_city(&alerts, "Mumbai");
    assert_eq!(mumbai.len(), 6);
    assert_eq!(mumbai[0]["source"], "current");
    assert_eq!(mumbai[0]["value"], 31.7);
    assert_eq!(mumbai[0]["country"], "India");
    assert_eq!(mumbai[0]["date"], "2026-07-17");
    let forecast_dates: Vec<_> = mumbai[1..].iter().map(|a| a["date"].clone()).collect();
    let expected: Vec<_> = (0..5).map(|d| json!(date_in(d))).collect();
    assert_eq!(forecast_dates, expected);
    assert!(mumbai[1..].iter().all(|a| a["source"] == "forecast"));

    let tokyo = of_city(&alerts, "Tokyo");
    let tokyo_forecast: Vec<_> = tokyo[1..]
        .iter()
        .map(|a| (a["date"].clone(), a["value"].clone()))
        .collect();
    assert_eq!(
        tokyo_forecast,
        vec![
            (json!(date_in(0)), json!(30.5)),
            (json!(date_in(1)), json!(31.0)),
            (json!(date_in(4)), json!(30.0)),
        ]
    );
    assert_eq!(alerts[0]["city"], "Mumbai");
}

#[tokio::test]
async fn response_uses_documented_camel_case_fields() {
    let pool = test_pool().await;
    let first = &alerts(&pool).await[0];
    let mut keys: Vec<_> = first.as_object().unwrap().keys().cloned().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "city",
            "country",
            "date",
            "severity",
            "source",
            "threshold",
            "type",
            "value"
        ]
    );
}

#[tokio::test]
async fn empty_database_returns_empty_array() {
    let pool = empty_pool().await;
    assert!(alerts(&pool).await.is_empty());
}

#[tokio::test]
async fn cold_wind_and_humidity_alerts_with_tiers() {
    let pool = empty_pool().await;
    post_weather(
        &pool,
        json!({"city": "Oslo", "country": "Norway", "temperatureCelsius": -16.0,
               "humidity": 90, "windSpeedKmh": 61.0}),
    )
    .await;
    let alerts = alerts(&pool).await;
    assert_eq!(alerts.len(), 3);
    let by_type = |t: &str| alerts.iter().find(|a| a["type"] == t).unwrap();
    assert_eq!(by_type("cold")["severity"], "severe");
    assert_eq!(by_type("cold")["threshold"], 0.0);
    assert_eq!(by_type("cold")["value"], -16.0);
    assert_eq!(by_type("wind")["severity"], "warning");
    assert_eq!(by_type("humidity")["severity"], "warning");
    assert_eq!(by_type("humidity")["value"], 90.0);
    assert!(alerts.iter().all(|a| a["source"] == "current"));
    assert_eq!(alerts[0]["severity"], "severe");
}

#[tokio::test]
async fn boundaries_are_inclusive_and_just_below_is_quiet() {
    let pool = empty_pool().await;
    post_weather(
        &pool,
        json!({"city": "Edge", "temperatureCelsius": 30.0, "humidity": 85, "windSpeedKmh": 40.0}),
    )
    .await;
    post_weather(
        &pool,
        json!({"city": "Calm", "temperatureCelsius": 29.9, "humidity": 84, "windSpeedKmh": 39.9}),
    )
    .await;
    let alerts = alerts(&pool).await;
    assert_eq!(of_city(&alerts, "Edge").len(), 3);
    assert!(of_city(&alerts, "Calm").is_empty());
}

#[tokio::test]
async fn only_latest_record_per_city_counts() {
    let pool = empty_pool().await;
    post_weather(
        &pool,
        json!({"city": "Cairo", "temperatureCelsius": 41.0, "recordedAt": "2026-10-01T10:00:00"}),
    )
    .await;
    post_weather(
        &pool,
        json!({"city": "cairo", "temperatureCelsius": 22.0, "recordedAt": "2026-10-02T10:00:00"}),
    )
    .await;
    assert!(alerts(&pool).await.is_empty());

    post_weather(
        &pool,
        json!({"city": "Cairo", "temperatureCelsius": 36.0, "recordedAt": "2026-10-03T10:00:00"}),
    )
    .await;
    let alerts = alerts(&pool).await;
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0]["severity"], "warning");
    assert_eq!(alerts[0]["date"], "2026-10-03");
}

#[tokio::test]
async fn city_without_forecast_gets_current_alerts_only() {
    let pool = empty_pool().await;
    post_weather(
        &pool,
        json!({"city": "Nowhere", "temperatureCelsius": 33.0}),
    )
    .await;
    let alerts = alerts(&pool).await;
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0]["source"], "current");
    assert!(alerts[0]["country"].is_null());
}

#[tokio::test]
async fn forecast_cold_uses_daily_low() {
    let pool = empty_pool().await;
    post_weather(&pool, json!({"city": "London", "temperatureCelsius": 10.0})).await;
    sqlx::query(
        "UPDATE forecasts SET low_temperature_celsius = -6.0 WHERE city_key = 'london' AND day_offset = 2",
    )
    .execute(&pool)
    .await
    .unwrap();
    let alerts = alerts(&pool).await;
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0]["type"], "cold");
    assert_eq!(alerts[0]["severity"], "warning");
    assert_eq!(alerts[0]["source"], "forecast");
    assert_eq!(alerts[0]["date"], date_in(2));
}

#[tokio::test]
async fn disabled_or_edited_thresholds_change_results() {
    let pool = test_pool().await;
    sqlx::query("UPDATE alert_thresholds SET enabled = 0 WHERE alert_type = 'heat'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(alerts(&pool).await.is_empty());

    sqlx::query(
        "UPDATE alert_thresholds SET enabled = 1, threshold = 31.5, warning_threshold = 35, severe_threshold = 40 \
         WHERE alert_type = 'heat'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let alerts = alerts(&pool).await;
    assert!(alerts.iter().all(|a| a["value"].as_f64().unwrap() >= 31.5));
    assert!(alerts.iter().all(|a| a["threshold"] == 31.5));
}

#[tokio::test]
async fn database_failure_returns_standard_error_body() {
    let pool = test_pool().await;
    sqlx::query("DROP TABLE alert_thresholds")
        .execute(&pool)
        .await
        .unwrap();
    let response = request(&pool, "GET", "/api/alerts", None).await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = json_body(response).await;
    assert_eq!(body["status"], 500);
    assert_eq!(body["path"], "/api/alerts");
    assert_eq!(body["message"], "An unexpected error occurred");
}

#[tokio::test]
async fn openapi_documents_alerts_endpoint() {
    let pool = test_pool().await;
    let spec = json_body(request(&pool, "GET", "/v3/api-docs", None).await).await;
    let get = &spec["paths"]["/api/alerts"]["get"];
    assert!(get.is_object());
    assert_eq!(get["tags"][0], "alerts");
    assert!(get["responses"]["200"].is_object());
    assert!(get["responses"]["500"].is_object());
    let alert = &spec["components"]["schemas"]["Alert"];
    for field in [
        "city",
        "type",
        "severity",
        "value",
        "threshold",
        "source",
        "date",
    ] {
        assert!(alert["properties"][field].is_object(), "missing {field}");
    }
}
