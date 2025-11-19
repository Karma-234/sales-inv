use std::collections::HashMap;

use crate::mauth::layers::{MyAuthLayer, MyAuthPermsLayer};
use crate::mproduct::models::ProductModel;
use crate::musers::handlers::{
    create_new_user_handler, delete_users_handler, get_users_handler, update_users_handler,
};
use crate::musers::schema::{DeleteUsersSchema, UpdateUsersSchema};
use crate::shared_var::FilterOptions;
use crate::{AppState, shared_var::MyBaseResponse};
use axum::Json;
use axum::extract::Query;
use axum::routing::{delete, post, put};
use axum::{Router, extract::State, routing::get};

pub fn create_user_router(app: AppState) -> Router {
    return Router::new()
        .route(
            "/health",
            get(|pool: axum::extract::State<AppState>| async move {
                let mut resp = HashMap::new();
                resp.insert("status".to_string(), "ok".to_string());
                let query_result = sqlx::query_as!(
                    ProductModel,
                    r#"SELECT * FROM products 
                    LIMIT 1"#
                )
                .fetch_one(&pool.0.db)
                .await;
                if let Some(_) = query_result.ok() {
                    resp.insert("db_status".to_string(), "connected".to_string());
                } else {
                    resp.insert("db_status".to_string(), "disconnected".to_string());
                }
                MyBaseResponse::ok(Some(resp), Some("healthy".into()));
            }),
        )
        .route(
            "/add",
            post(
                |pool: axum::extract::State<AppState>,
                 payload: axum::Json<crate::musers::schema::AddUserSchema>| async move {
                    let app = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return create_new_user_handler(State(app), payload).await;
                },
            )
            .layer(MyAuthPermsLayer {}),
        )
        .route(
            "/get",
            get(
                |State(pool): State<AppState>, Query(opts): Query<FilterOptions>| async move {
                    let app = AppState {
                        db: pool.db.clone(),
                        env: pool.env,
                    };
                    let query_opts = Query(Some(opts));
                    return get_users_handler(State(app), query_opts).await;
                },
            )
            .layer(MyAuthPermsLayer {}),
        )
        .route(
            "/update",
            put(
                |State(pool): State<AppState>, Json(payload): Json<UpdateUsersSchema>| async move {
                    let app = AppState {
                        db: pool.db.clone(),
                        env: pool.env,
                    };
                    let data = Json(payload);
                    return update_users_handler(State(app), data).await;
                },
            )
            .layer(MyAuthPermsLayer {}),
        )
        .route(
            "/delete",
            delete(
                |State(pool): State<AppState>, Json(payload): Json<DeleteUsersSchema>| async move {
                    let app = AppState {
                        db: pool.db.clone(),
                        env: pool.env,
                    };
                    let data = Json(payload);
                    return delete_users_handler(State(app), data).await;
                },
            )
            .layer(MyAuthPermsLayer {}),
        )
        .layer(MyAuthLayer { state: app.clone() })
        .with_state(app);
}
