use std::collections::HashMap;

use crate::mproduct::models::ProductModel;
use crate::musers::handlers::{create_new_user_handler, get_users_handler, update_users_handler};
use crate::musers::schema::UpdateUsersSchema;
use crate::shared_var::FilterOptions;
use crate::{AppState, shared_var::MyBaseResponse};
use axum::Json;
use axum::extract::Query;
use axum::routing::{post, put};
use axum::{Router, extract::State, routing::get};
use sqlx::{Pool, Postgres};

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
                    MyBaseResponse::ok(Some(resp), Some("healthy".into()));
                },
            ),
        )
        .route(
            "/users/add",
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::Json<crate::musers::schema::AddUserSchema>| async move {
                    let app = AppState { db: pool.0 };
                    return create_new_user_handler(State(app), payload).await;
                },
            ),
        )
        .route(
            "/users/get",
            get(
                |State(pool): State<Pool<Postgres>>, Query(opts): Query<FilterOptions>| async move {
                    let app = AppState { db: pool.clone() };
                    let query_opts = Query(Some(opts));
                    return get_users_handler(State(app), query_opts).await;
                },
            ),
        )
        .route(
            "/users/update",
            put(
                |State(pool): State<Pool<Postgres>>, Json(payload): Json<UpdateUsersSchema>| async move {
                    let app = AppState { db: pool.clone() };
                    let data = Json(payload);
                    return update_users_handler(State(app), data).await;
                },
            ),
        )
        .with_state(app.db.clone());
}
