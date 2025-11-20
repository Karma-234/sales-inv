use crate::AppState;
use crate::mcart::models::{CartItemModel, CartModel, CartWithItemsModel};
use crate::mcart::schemas::{
    AddCartItemSchema, CreateCartSchema, GetCartByUserSchema, UpdateCartItemSchema,
};
use crate::mcart::sql_string::CartSQLString;
use crate::mproduct::models::ProductModel;
use crate::shared_var::MyBaseResponse;

use sqlx::query_as;

#[utoipa::path(
    post,
    path = "/api/v1/cart/create", 
    tag = "Carts",
    request_body = CreateCartSchema,
    responses(
        (status = 200, description = "Carts created successfully", body = MyBaseResponse<Vec<CartModel>>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartModel>),
    ),
     security(("bearerAuth" = [])), 
)]
pub async fn create_cart_handler(
    payload: axum::extract::Json<CreateCartSchema>,
    state: AppState,
) -> MyBaseResponse<CartModel> {
    let res = query_as::<_, CartModel>(CartSQLString::CREATE_CART_ID)
        .bind(&payload.user_id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart created successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/cart/get-by-user", 
    tag = "Carts",
    request_body = GetCartByUserSchema,
    responses(
        (status = 200, description = "Carts retrieved successfully", body = MyBaseResponse<Vec<CartWithItemsModel>>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartWithItemsModel>),
    )
     
)]
pub async fn get_cart_by_user_handler(
    payload: axum::extract::Json<GetCartByUserSchema>,
    state: AppState,
) -> MyBaseResponse<Vec<CartWithItemsModel>> {
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_CART_BY_USER_ID)
        .bind(&payload.user_id)
        .fetch_all(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart retrieved successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/cart/get-open-by-user", 
    tag = "Carts",
    request_body = GetCartByUserSchema,
    responses(
        (status = 200, description = "Open cart retrieved successfully", body = MyBaseResponse<CartWithItemsModel>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartWithItemsModel>),
    )
     
)]
pub async fn get_open_cart_by_user_handler(
    payload: axum::extract::Json<GetCartByUserSchema>,
    state: AppState,
) -> MyBaseResponse<CartWithItemsModel> {
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_OPEN_CART_BY_USER_ID)
        .bind(&payload.user_id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Cart retrieved successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/cart/add-item", 
    tag = "Carts",
    request_body = AddCartItemSchema,
    responses(
        (status = 200, description = "Item added to cart successfully", body = MyBaseResponse<CartItemModel>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartItemModel>),
    )
     
)]
pub async fn add_item_to_cart_handler(
    payload: axum::extract::Json<AddCartItemSchema>,
    state: AppState,
) -> MyBaseResponse<CartItemModel> {
        if payload.quantity <= 0 {
        return MyBaseResponse::error(400, "Quantity cannot be negative");
    }
    let tx = state.db.begin().await;
    let mut tx = match tx {
        Ok(t)=>t,
        Err(e) => {return MyBaseResponse::db_err(e)}
    };
        

    let product = query_as::<_, ProductModel>(r#"
        SELECT *
        FROM products
        WHERE id = $1
        "#)
        .bind(&payload.cart_id)
        .bind(&payload.product_id)
        .fetch_optional(&mut *tx)
        .await;
       let stock =  match product{
            Ok(p)=>p,
            
            Err(e)=>{
                let _ = tx.rollback().await;
                
                return MyBaseResponse::db_err(e)
            }
        };
        if stock.unwrap().quantity < payload.quantity {
            let _ = tx.rollback().await;
            return MyBaseResponse::error(400, "Insufficient product stock");
        }
   
    let res = query_as::<_, CartItemModel>(CartSQLString::INSERT_CART_ITEM)
        .bind(&payload.cart_id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .bind(&payload.unit_amount)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(cart) => MyBaseResponse::ok(Some(cart), Some("Item added to cart successfully".into())),
        Err(e) => MyBaseResponse::db_err(e),
    }
}
#[utoipa::path(
    put,
    path = "/api/v1/cart/update-item", 
    tag = "Carts",
    request_body = UpdateCartItemSchema,
    responses(
        (status = 200, description = "Item updated in cart successfully", body = MyBaseResponse<CartItemModel>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartItemModel>),
    )
     
)]
pub async fn update_item_in_cart_handler(
    payload: axum::extract::Json<UpdateCartItemSchema>,
    state: AppState,
) -> MyBaseResponse<CartItemModel> {
        if payload.quantity < 0 {
        return MyBaseResponse::error(400, "Quantity cannot be negative");
    }
    let res = query_as::<_, CartItemModel>(CartSQLString::UPSERT_CART_ITEM)
        .bind(&payload.cart_id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .bind(&payload.unit_amount)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(cart) => {
            MyBaseResponse::ok(Some(cart), Some("Item updated in cart successfully".into()))
        }
        Err(e) => MyBaseResponse::db_err(e),
    }
}
