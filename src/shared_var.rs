use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

use crate::mproduct::schema::{AddProductSchema, DeleteProductSchema, UpdateProductSchema};
use crate::{AppState, mproduct};

#[derive(serde::Serialize, Debug, Clone)]
#[serde(bound = "T: serde::Serialize")]
pub struct MyBaseResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> MyBaseResponse<T> {
    /// Convenience constructor for successful responses.
    pub fn ok(data: Option<T>, message: Option<String>) -> Self {
        Self {
            code: 200,
            message: message.unwrap_or("OK".into()),
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
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct FilterOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            &mproduct::routes::get_products(),
            get(
                // mproduct::api::get_product_handler,
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 filter: axum::extract::Query<FilterOptions>| async move {
                    // construct AppState from the shared Pool and forward to your handler
                    let op = Query(FilterOptions {
                        limit: Some(2),
                        page: Some(3),
                        // search: Some("Amoxil".to_string()),
                        search: Some(filter.0.search.unwrap_or_default()),
                    });
                    let state = AppState { db: pool.0 };
                    mproduct::handlers::get_product_handler(op, State(state)).await
                },
            ),
        )
        .route(
            &mproduct::routes::mock(),
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState { db: pool.0 };
                    mproduct::handlers::mock_post_handler(State(state), payload).await
                },
            ),
        )
        .route(
            &mproduct::routes::update_product(),
            put(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<UpdateProductSchema>| async move {
                    let state = AppState { db: pool.0 };
                    mproduct::handlers::update_prod_handler(State(state), payload).await
                },
            ),
        )
        .route(
            &mproduct::routes::del_product(),
            delete(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<DeleteProductSchema>| async move {
                    let state = AppState { db: pool.0 };
                    mproduct::handlers::del_prod_handler(payload, State(state)).await
                },
            ),
        )
        .route(
            &mproduct::routes::add_product(),
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState { db: pool.0 };
                    mproduct::handlers::add_product_handler(payload, State(state)).await;
                },
            ),
        )
        .with_state(app_state.db)
}
