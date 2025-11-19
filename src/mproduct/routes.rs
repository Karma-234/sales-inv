use axum::{
    Router,
    extract::{Query, State},
    routing::{delete, get, post, put},
};

use crate::{
    AppState,
    mauth::layers::{MyAuthLayer, MyAuthPermsLayer},
    mproduct::{
        self,
        schema::{AddProductSchema, DeleteProductSchema, UpdateProductSchema},
    },
    shared_var::FilterOptions,
};

pub fn create_prod_router(app: AppState) -> Router {
    return Router::new()
        .route(
            "/get",
            get(
                |pool: axum::extract::State<AppState>,
                 filter: axum::extract::Query<FilterOptions>| async move {
                    let op = Query(FilterOptions {
                        limit: filter.limit.as_ref().and_then(|f| Some(f.clone())),
                        page: filter.page.as_ref().and_then(|f| Some(f.clone())),
                        // search: Some("Amoxil".to_string()),
                        search: filter.search.as_ref().and_then(|f| Some(f.clone())),
                    });
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::get_product_handler(op, State(state)).await;
                },
            ),
        )
        .route(
            "/update",
            put(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<UpdateProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::update_product_handler(State(state), payload).await;
                },
            )
            .layer(MyAuthPermsLayer {}),
        )
        .route(
            "/delete",
            delete(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<DeleteProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::del_product_handler(payload, State(state)).await;
                },
            ),
        )
        .route(
            "/add",
            post(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::add_product_handler(payload, State(state)).await;
                },
            ),
        )
        .layer(MyAuthLayer { state: app.clone() })
        .with_state(app);
}
