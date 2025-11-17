use std::collections::HashMap;

use crate::mproduct::models::ProductModel;
use crate::{AppState, shared_var::MyBaseResponse};
use axum::{Router, extract::State, routing::get};

pub fn create_user_router(app: State<AppState>) -> Router {
    return Router::new()
        .route(
            "/health",
            get(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>| async move {
                    let mut resp = HashMap::new();
                    resp.insert("status".to_string(), "ok".to_string());
                    let query_result = sqlx::query_as!(
                        ProductModel,
                        r#"SELECT * FROM products 
                    LIMIT 1"#
                    )
                    .fetch_one(&pool.0)
                    .await;
                    if let Some(_) = query_result.ok() {
                        resp.insert("db_status".to_string(), "connected".to_string());
                    } else {
                        resp.insert("db_status".to_string(), "disconnected".to_string());
                    }
                    MyBaseResponse::ok(Some(resp), Some("healthy".into()))
                },
            ),
        )
        .with_state(app.db.clone());
}
