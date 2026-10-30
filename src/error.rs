use crate::models::ErrorResponse;
use axum::{
    Json,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use chrono::Utc;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
    path: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            path: String::new(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
            path: String::new(),
        }
    }

    fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "An unexpected error occurred".to_owned(),
            path: String::new(),
        }
    }

    fn at_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    fn into_error_response(self) -> ErrorResponse {
        ErrorResponse {
            timestamp: Utc::now().naive_utc(),
            status: self.status.as_u16(),
            error: self
                .status
                .canonical_reason()
                .unwrap_or("Internal Server Error")
                .to_owned(),
            message: self.message,
            path: self.path,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        (status, Json(self.into_error_response())).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!(%error, "database operation failed");
        Self::internal()
    }
}

pub fn json_error(error: axum::extract::rejection::JsonRejection, uri: &Uri) -> ApiError {
    ApiError::bad_request(error.body_text()).at_path(uri.path())
}

pub fn bad_path_id(uri: &Uri) -> ApiError {
    ApiError::bad_request("Path id must be an integer").at_path(uri.path())
}

pub fn with_path(error: ApiError, uri: &Uri) -> ApiError {
    error.at_path(uri.path())
}
