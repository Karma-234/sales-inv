use crate::AppState;
use crate::mcart::models::{CartModel, CartWithItemsModel};
use crate::mcart::schemas::{CreateCartSchema, GetCartByUserSchema};
use crate::mcart::sql_string::CartSQLString;
use crate::shared_var::MyBaseResponse;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::query_as;
pub async fn create_cart_handler(
    payload: axum::extract::Json<CreateCartSchema>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = query_as::<_, CartModel>(CartSQLString::CREATE_CART_ID)
        .bind(&payload.user_id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart created successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}

pub async fn get_cart_by_user_handler(
    payload: axum::extract::Json<GetCartByUserSchema>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_CART_BY_USER_ID)
        .bind(&payload.user_id)
        .fetch_all(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart retrieved successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}

pub async fn get_open_cart_by_user_handler(
    payload: axum::extract::Json<GetCartByUserSchema>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_OPEN_CART_BY_USER_ID)
        .bind(&payload.user_id)
        .fetch_all(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart retrieved successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}
