use axum::{
    Router,
    extract::Query,
    extract::State,
    routing::{delete, get, post, put},
};

use crate::{
    AppState,
    config::Config,
    mproduct::{
        self,
        schema::{AddProductSchema, DeleteProductSchema, UpdateProductSchema},
    },
    shared_var::FilterOptions,
};

pub fn get_products() -> String {
    String::from("/products")
}
pub fn add_product() -> String {
    String::from("/products")
}
pub fn mock() -> String {
    String::from("/mock")
}
pub fn update_product() -> String {
    String::from("/products")
}

pub fn del_product() -> String {
    String::from("/products")
}

pub fn create_prod_router(app: State<AppState>) -> Router {
    return Router::new()
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
                    let state = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return mproduct::handlers::get_product_handler(op, State(state)).await;
                },
            ),
        )
        .route(
            &mproduct::routes::mock(),
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return mproduct::handlers::mock_post_handler(State(state), payload).await;
                },
            ),
        )
        .route(
            &mproduct::routes::update_product(),
            put(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<UpdateProductSchema>| async move {
                    let state = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return mproduct::handlers::update_prod_handler(State(state), payload).await;
                },
            ),
        )
        .route(
            &mproduct::routes::del_product(),
            delete(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<DeleteProductSchema>| async move {
                    let state = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return mproduct::handlers::del_prod_handler(payload, State(state)).await;
                },
            ),
        )
        .route(
            &mproduct::routes::add_product(),
            post(
                |pool: axum::extract::State<sqlx::Pool<sqlx::Postgres>>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState {
                        db: pool.0,
                        env: Config::init(),
                    };
                    return mproduct::handlers::add_product_handler(payload, State(state)).await;
                },
            ),
        )
        .with_state(app.db.clone());
}
