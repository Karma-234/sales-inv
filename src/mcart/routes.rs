use crate::{
    AppState,
    mauth::layers::MyAuthLayer,
    mcart::{
        self,
        schemas::{AddCartItemSchema, UpdateCartItemSchema},
    },
};
use axum::{
    Json, Router,
    body::Body,
    extract::{Request, State},
    routing::{delete, get, post, put},
};

pub fn create_cart_router(app: AppState) -> Router {
    return Router::new()
        .route(
            "/create",
            post(|pool: State<AppState>, request: Request<Body>| async move {
                return mcart::handlers::create_cart_handler(request, pool.0.clone()).await;
            }),
        )
        .route(
            "/add-item",
            post(
                |pool: State<AppState>, payload: Json<AddCartItemSchema>| async move {
                    return mcart::handlers::add_item_to_cart_handler(payload, pool.0.clone())
                        .await;
                },
            ),
        )
        .route(
            "/update-item",
            put(
                |pool: State<AppState>, payload: Json<UpdateCartItemSchema>| async move {
                    return mcart::handlers::update_item_in_cart_handler(payload, pool.0.clone())
                        .await;
                },
            ),
        )
        .route(
            "/get-by-user",
            get(|pool: State<AppState>, request: Request<Body>| async move {
                return mcart::handlers::get_cart_by_user_handler(request, pool.0.clone()).await;
            }),
        )
        .route(
            "/get-open-by-user",
            get(|pool: State<AppState>, request: Request<Body>| async move {
                return mcart::handlers::get_open_cart_by_user_handler(request, pool.0.clone())
                    .await;
            }),
        )
        .route(
            "/delete-items",
            delete(
                |pool: State<AppState>, payload: Json<Vec<UpdateCartItemSchema>>| async move {
                    return mcart::handlers::delete_items_from_cart_handler(
                        payload,
                        pool.0.clone(),
                    )
                    .await;
                },
            ),
        )
        .layer(MyAuthLayer { state: app.clone() })
        .with_state(app);
}
