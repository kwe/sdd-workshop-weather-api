use axum::{
    Router,
    extract::Path,
    http::{HeaderName, HeaderValue, StatusCode, header},
    middleware::map_response,
    response::{IntoResponse, Response},
    routing::get,
};
use sqlx::SqlitePool;

pub const CONTENT_SECURITY_POLICY: &str = "default-src 'none'; \
script-src 'self' https://cdn.jsdelivr.net; \
style-src 'self' 'unsafe-inline'; \
img-src 'self' data: blob: https://cdn.jsdelivr.net; \
connect-src 'self'; \
font-src 'self'; \
worker-src 'self' blob:; \
object-src 'none'; \
base-uri 'self'; \
form-action 'none'; \
frame-ancestors 'none'";

const HTML: &str = "text/html; charset=utf-8";
const JS: &str = "text/javascript; charset=utf-8";
const CSS: &str = "text/css; charset=utf-8";
const SVG: &str = "image/svg+xml";

pub struct Asset {
    pub path: &'static str,
    pub content_type: &'static str,
    pub body: &'static str,
}

macro_rules! asset {
    ($path:literal, $ct:expr) => {
        Asset {
            path: $path,
            content_type: $ct,
            body: include_str!(concat!("../static/", $path)),
        }
    };
}

pub const INDEX: Asset = asset!("index.html", HTML);

/// Every file the frontend serves; lookups are exact matches on this table.
pub const ASSETS: &[Asset] = &[
    asset!("css/app.css", CSS),
    asset!("favicon.svg", SVG),
    asset!("js/main.js", JS),
    asset!("js/state.js", JS),
    asset!("js/api.js", JS),
    asset!("js/gazetteer.js", JS),
    asset!("js/weather.js", JS),
    asset!("js/sun.js", JS),
    asset!("js/dom.js", JS),
    asset!("js/globe-scene.js", JS),
    asset!("js/ui.js", JS),
];

fn serve(asset: &'static Asset) -> Response {
    ([(header::CONTENT_TYPE, asset.content_type)], asset.body).into_response()
}

async fn index() -> Response {
    serve(&INDEX)
}

async fn static_asset(Path(path): Path<String>) -> Response {
    match ASSETS.iter().find(|asset| asset.path == path) {
        Some(asset) => serve(asset),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn security_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CONTENT_SECURITY_POLICY),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    response
}

/// Frontend routes; security headers apply to these routes only.
pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(index))
        .route("/static/{*path}", get(static_asset))
        .layer(map_response(security_headers))
}
