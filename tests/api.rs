use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use http_body_util::BodyExt;
use sdd_workshop_weather_api::{app, db};
use serde_json::{Value, json};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite database should open");
    db::migrate(&pool)
        .await
        .expect("database migrations should succeed");
    app(pool)
}

async fn request(app: axum::Router, method: &str, uri: &str, body: Option<Value>) -> Response {
    let mut builder = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(body) => {
            builder = builder.header("content-type", "application/json");
            Body::from(body.to_string())
        }
        None => Body::empty(),
    };
    app.oneshot(builder.body(body).expect("request should be valid"))
        .await
        .expect("router should handle the request")
}

async fn response_json(response: Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body should be readable")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response should contain JSON")
}

#[tokio::test]
async fn lists_seeded_records_and_filters_city_case_insensitively() {
    let app = test_app().await;
    let response = request(app.clone(), "GET", "/api/weather", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response_json(response).await.as_array().unwrap().len(), 10);

    let response = request(app.clone(), "GET", "/api/weather?city=lOnDoN", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
    assert_eq!(body[0]["city"], "London");

    let response = request(app, "GET", "/api/weather?city=s%C3%A3o%20paulo", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
    assert_eq!(body[0]["city"], "São Paulo");
}

#[tokio::test]
async fn creates_retrieves_and_deletes_records() {
    let app = test_app().await;
    let response = request(
        app.clone(),
        "POST",
        "/api/weather",
        Some(json!({
            "city": "Madrid",
            "country": "Spain",
            "temperatureCelsius": 29.4,
            "humidity": 35,
            "description": "Sunny",
            "windSpeedKmh": 7.2,
            "recordedAt": "2026-07-17T10:00:00"
        })),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let created = response_json(response).await;
    assert_eq!(created["city"], "Madrid");
    assert_eq!(created["recordedAt"], "2026-07-17T10:00:00");
    let id = created["id"].as_i64().unwrap();

    let response = request(app.clone(), "GET", &format!("/api/weather/{id}"), None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response_json(response).await["id"], id);

    let response = request(app.clone(), "DELETE", &format!("/api/weather/{id}"), None).await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let response = request(app, "GET", &format!("/api/weather/{id}"), None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error = response_json(response).await;
    assert_eq!(
        error["message"],
        format!("Weather record not found for id {id}")
    );
    assert_eq!(error["path"], format!("/api/weather/{id}"));
}

#[tokio::test]
async fn validation_errors_use_the_standard_error_response() {
    let app = test_app().await;
    let response = request(
        app,
        "POST",
        "/api/weather",
        Some(json!({
            "city": "Madrid",
            "temperatureCelsius": 75,
            "humidity": 101
        })),
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["error"], "Bad Request");
    assert_eq!(
        error["message"],
        "Temperature must be between -90 and 60 Celsius"
    );
    assert_eq!(error["path"], "/api/weather");
}

#[tokio::test]
async fn rejects_humidity_outside_its_allowed_range() {
    let app = test_app().await;
    let response = request(
        app,
        "POST",
        "/api/weather",
        Some(json!({"city": "Madrid", "humidity": 101})),
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["message"], "Humidity must be between 0 and 100");
}

#[tokio::test]
async fn malformed_json_uses_the_standard_error_response() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/weather")
                .header("content-type", "application/json")
                .body(Body::from("{"))
                .expect("request should be valid"),
        )
        .await
        .expect("router should handle the request");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["error"], "Bad Request");
    assert_eq!(error["path"], "/api/weather");
}

#[tokio::test]
async fn serves_openapi_documentation() {
    let app = test_app().await;
    let response = request(app, "GET", "/v3/api-docs", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let document = response_json(response).await;
    assert!(document["paths"]["/api/weather"].is_object());
    assert!(document["paths"]["/api/weather/{id}"].is_object());
}

#[tokio::test]
async fn swagger_ui_legacy_path_redirects_to_the_ui() {
    let app = test_app().await;
    let response = request(app, "GET", "/swagger-ui.html", None).await;
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.headers().get("location").unwrap(), "/swagger-ui/");
}

#[tokio::test]
async fn rejects_blank_city_and_non_numeric_path_ids_with_standard_errors() {
    let app = test_app().await;
    let response = request(
        app.clone(),
        "POST",
        "/api/weather",
        Some(json!({"city": "  "})),
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["message"], "City is required");

    let response = request(app.clone(), "GET", "/api/weather/not-an-id", None).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["path"], "/api/weather/not-an-id");

    let response = request(app.clone(), "GET", "/api/weather/0", None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response = request(app, "GET", "/api/weather/-1", None).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn defaults_recorded_at_when_it_is_omitted() {
    let app = test_app().await;
    let response = request(app, "POST", "/api/weather", Some(json!({"city": "Madrid"}))).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let created = response_json(response).await;
    assert_eq!(created["city"], "Madrid");
    assert!(!created["recordedAt"].as_str().unwrap().is_empty());
}
