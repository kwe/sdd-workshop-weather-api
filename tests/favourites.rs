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

async fn request(app: &axum::Router, method: &str, uri: &str, body: Option<Value>) -> Response {
    let mut builder = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(body) => {
            builder = builder.header("content-type", "application/json");
            Body::from(body.to_string())
        }
        None => Body::empty(),
    };
    app.clone()
        .oneshot(builder.body(body).expect("request should be valid"))
        .await
        .expect("router should handle the request")
}

async fn raw_post(app: &axum::Router, body: &'static str) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/favourites")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .expect("request should be valid"),
        )
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

async fn star(app: &axum::Router, city: &str) -> Response {
    request(
        app,
        "POST",
        "/api/favourites",
        Some(json!({ "city": city })),
    )
    .await
}

async fn list(app: &axum::Router) -> Vec<Value> {
    let response = request(app, "GET", "/api/favourites", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await.as_array().unwrap().clone()
}

#[tokio::test]
async fn starts_empty() {
    let app = test_app().await;
    assert!(list(&app).await.is_empty());
}

#[tokio::test]
async fn create_returns_201_then_200_without_duplicating() {
    let app = test_app().await;

    let first = star(&app, "London").await;
    assert_eq!(first.status(), StatusCode::CREATED);
    let first = response_json(first).await;
    assert_eq!(first["city"], "London");
    assert_eq!(first["weather"]["city"], "London");
    assert!(first["favouritedAt"].is_string());

    let second = star(&app, "  lOnDoN  ").await;
    assert_eq!(second.status(), StatusCode::OK);
    let second = response_json(second).await;
    assert_eq!(second["city"], "London");
    assert_eq!(second["favouritedAt"], first["favouritedAt"]);

    assert_eq!(list(&app).await.len(), 1);
}

#[tokio::test]
async fn unknown_city_is_404_and_not_stored() {
    let app = test_app().await;
    let response = star(&app, "Atlantis").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response_json(response).await;
    assert_eq!(body["status"], 404);
    assert_eq!(body["path"], "/api/favourites");
    assert!(list(&app).await.is_empty());
}

#[tokio::test]
async fn invalid_input_is_400() {
    let app = test_app().await;
    let too_long = "x".repeat(101);
    let just_fits = "x".repeat(100);

    for body in [
        json!({}),
        json!({ "city": null }),
        json!({ "city": "" }),
        json!({ "city": "   " }),
        json!({ "city": 42 }),
        json!({ "city": ["London"] }),
        json!({ "city": too_long }),
    ] {
        let response = request(&app, "POST", "/api/favourites", Some(body.clone())).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{body}");
        assert_eq!(response_json(response).await["status"], 400);
    }
    assert_eq!(
        raw_post(&app, "{not json").await.status(),
        StatusCode::BAD_REQUEST
    );

    // exactly 100 characters passes validation and then fails on lookup
    assert_eq!(star(&app, &just_fits).await.status(), StatusCode::NOT_FOUND);
    assert!(list(&app).await.is_empty());
}

#[tokio::test]
async fn list_is_newest_first_with_latest_weather() {
    let app = test_app().await;
    let older = request(
        &app,
        "POST",
        "/api/weather",
        Some(json!({"city": "Paris", "temperatureCelsius": 1.0, "recordedAt": "2020-01-01T00:00:00"})),
    )
    .await;
    assert_eq!(older.status(), StatusCode::CREATED);
    // Same recordedAt as the seeded Paris row, inserted later: higher id wins the tie.
    let tie = request(
        &app,
        "POST",
        "/api/weather",
        Some(json!({"city": "paris", "temperatureCelsius": 5.5, "recordedAt": "2026-07-17T09:00:00"})),
    )
    .await;
    assert_eq!(tie.status(), StatusCode::CREATED);

    assert_eq!(star(&app, "Tokyo").await.status(), StatusCode::CREATED);
    assert_eq!(star(&app, "Paris").await.status(), StatusCode::CREATED);

    let favourites = list(&app).await;
    let cities: Vec<_> = favourites.iter().map(|f| f["city"].clone()).collect();
    assert_eq!(cities, [json!("paris"), json!("Tokyo")]);
    assert_eq!(favourites[0]["weather"]["temperatureCelsius"], 5.5);
    assert_eq!(favourites[1]["weather"]["description"], "Humid");
}

#[tokio::test]
async fn favourite_without_records_is_listed_with_null_weather() {
    let app = test_app().await;
    let created = response_json(
        request(
            &app,
            "POST",
            "/api/weather",
            Some(json!({"city": "Gotham"})),
        )
        .await,
    )
    .await;
    assert_eq!(star(&app, "Gotham").await.status(), StatusCode::CREATED);
    let id = created["id"].as_i64().unwrap();
    let deleted = request(&app, "DELETE", &format!("/api/weather/{id}"), None).await;
    assert_eq!(deleted.status(), StatusCode::NO_CONTENT);

    let favourites = list(&app).await;
    assert_eq!(favourites.len(), 1);
    assert_eq!(favourites[0]["city"], "Gotham");
    assert!(favourites[0]["weather"].is_null());

    let removed = request(&app, "DELETE", "/api/favourites/Gotham", None).await;
    assert_eq!(removed.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_removes_then_404s_and_is_case_and_encoding_insensitive() {
    let app = test_app().await;
    assert_eq!(star(&app, "São Paulo").await.status(), StatusCode::CREATED);
    assert_eq!(star(&app, "Berlin").await.status(), StatusCode::CREATED);

    let response = request(&app, "DELETE", "/api/favourites/s%C3%83O%20paulo", None).await;
    // Unicode-aware: "SÃO" lower-cases to "são"
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let again = request(&app, "DELETE", "/api/favourites/S%C3%A3o%20Paulo", None).await;
    assert_eq!(again.status(), StatusCode::NOT_FOUND);
    assert_eq!(response_json(again).await["status"], 404);

    let berlin = request(&app, "DELETE", "/api/favourites/BERLIN", None).await;
    assert_eq!(berlin.status(), StatusCode::NO_CONTENT);
    assert!(list(&app).await.is_empty());

    let never = request(&app, "DELETE", "/api/favourites/Tokyo", None).await;
    assert_eq!(never.status(), StatusCode::NOT_FOUND);
    let blank = request(&app, "DELETE", "/api/favourites/%20%20", None).await;
    assert_eq!(blank.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn openapi_documents_favourites() {
    let app = test_app().await;
    let doc = response_json(request(&app, "GET", "/v3/api-docs", None).await).await;
    let list = &doc["paths"]["/api/favourites"];
    assert!(list["get"]["responses"]["200"].is_object());
    for status in ["200", "201", "400", "404"] {
        assert!(list["post"]["responses"][status].is_object(), "{status}");
    }
    let item = &doc["paths"]["/api/favourites/{city}"]["delete"]["responses"];
    for status in ["204", "400", "404"] {
        assert!(item[status].is_object(), "{status}");
    }
    assert!(doc["components"]["schemas"]["Favourite"].is_object());
}
