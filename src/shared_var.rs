use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(serde::Serialize, Debug, Clone)]
#[serde(bound = "T: serde::Serialize")]
pub struct MyBaseResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> MyBaseResponse<T> {
    /// Convenience constructor for successful responses.
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: 200,
            message: "OK".to_string(),
            data,
        }
    }

    /// Convenience constructor for error responses.
    pub fn error(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T> IntoResponse for MyBaseResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        // Map `code` (u32) to an HTTP status if possible, otherwise default to 200
        let status = StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::OK);
        (status, Json(self)).into_response()
    }
}

pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "ADMIN",
            UserRole::User => "USER",
            UserRole::Guest => "GUEST",
        }
    }
}
