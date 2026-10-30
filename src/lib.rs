pub mod db;
pub mod error;
pub mod favourites;
pub mod frontend;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod service;

use axum::{Router, routing::get};
use sqlx::SqlitePool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_weather_records,
        handlers::get_weather_record,
        handlers::create_weather_record,
        handlers::delete_weather_record,
        handlers::get_forecast,
        favourites::get_favourites,
        favourites::create_favourite,
        favourites::delete_favourite
    ),
    components(schemas(
        models::WeatherRecord,
        models::ForecastDay,
        models::CreateWeatherRecordRequest,
        models::ErrorResponse,
        favourites::Favourite,
        favourites::CreateFavouriteRequest
    )),
    tags(
        (name = "weather", description = "Weather record operations"),
        (name = "favourites", description = "Favourite cities")
    )
)]
struct ApiDoc;

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/api/weather",
            get(handlers::get_weather_records).post(handlers::create_weather_record),
        )
        .route("/api/weather/forecast/{city}", get(handlers::get_forecast))
        .route(
            "/api/weather/{id}",
            get(handlers::get_weather_record).delete(handlers::delete_weather_record),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/v3/api-docs", ApiDoc::openapi()))
        .route(
            "/swagger-ui.html",
            get(|| async { axum::response::Redirect::temporary("/swagger-ui/") }),
        )
        .merge(frontend::router())
        .merge(favourites::router())
        .with_state(pool)
}
