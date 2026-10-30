use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    response::Response,
};
use http_body_util::BodyExt;
use sdd_workshop_weather_api::{app, db, frontend};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn get(uri: &str) -> Response {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite database should open");
    db::migrate(&pool).await.expect("migrations should succeed");
    app(pool)
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should handle the request")
}

async fn body_text(response: Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn asset(path: &str) -> &'static str {
    frontend::ASSETS
        .iter()
        .find(|a| a.path == path)
        .unwrap_or_else(|| panic!("{path} should be a served asset"))
        .body
}

#[tokio::test]
async fn alerts_module_is_served_as_javascript_with_csp() {
    let response = get("/static/js/alerts.js").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/javascript; charset=utf-8"
    );
    assert_eq!(
        response.headers()[header::CONTENT_SECURITY_POLICY],
        frontend::CONTENT_SECURITY_POLICY
    );
    assert!(body_text(response).await.contains("renderAlertsBanner"));
}

#[tokio::test]
async fn index_contains_alert_containers() {
    let html = body_text(get("/").await).await;
    assert!(html.contains(r#"id="alerts-banner""#));
    assert!(html.contains(r#"id="city-alerts""#));
    assert!(!html.contains("<script>"), "no inline scripts");
}

#[test]
fn alert_code_never_builds_html_from_strings() {
    const FORBIDDEN: &[&str] = &[
        "innerHTML",
        "outerHTML",
        "insertAdjacentHTML",
        "document.write",
        "eval(",
        "new Function",
    ];
    for path in [
        "js/alerts.js",
        "js/ui.js",
        "js/main.js",
        "js/globe-scene.js",
    ] {
        let body = asset(path);
        for token in FORBIDDEN {
            assert!(!body.contains(token), "{path} must not use {token}");
        }
    }
}

#[test]
fn alerts_refresh_reuses_the_existing_timer() {
    let main = asset("js/main.js");
    assert_eq!(main.matches("setInterval(").count(), 1);
    assert!(main.contains("refreshAlerts(ui)"));
    assert!(asset("js/api.js").contains("/api/alerts"));
}

#[test]
fn reduced_motion_disables_alert_animation() {
    let css = asset("css/app.css");
    let block = css
        .split("@media (prefers-reduced-motion: reduce)")
        .nth(1)
        .expect("reduced-motion block should exist");
    assert!(block.contains(".alerts-dot"));
    let scene = asset("js/globe-scene.js");
    assert!(scene.contains("const alertRings = reduced"));
    assert!(scene.contains("setAlertRings"));
}
