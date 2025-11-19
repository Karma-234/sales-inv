use crate::AppState;
use crate::mauth::handlers::user_login_handler;
use crate::mauth::schemas::LoginUserSchema;
use axum::Router;
use axum::extract::State;
use axum::routing::post;

pub fn create_auth_router(app: AppState) -> Router {
    return Router::new()
        .route(
            "/login",
            post(
                |pool: axum::extract::State<AppState>,
                 payload: axum::Json<LoginUserSchema>| async move {
                    let app = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return user_login_handler(State(app), payload).await;
                },
            ),
        )
        .with_state(app);
}
