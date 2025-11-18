use crate::AppState;
use crate::config::Config;
use crate::mauth::handlers::user_login_handler;
use crate::mauth::schemas::LoginUserSchema;
use axum::Router;
use axum::extract::State;
use axum::routing::post;

pub fn create_auth_router(app: State<AppState>) -> Router {
    return Router::new()
        .route(
            "/login",
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::Json<LoginUserSchema>| async move {
                    let app = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return user_login_handler(State(app), payload).await;
                },
            ),
        )
        .with_state(app.db.clone());
}
