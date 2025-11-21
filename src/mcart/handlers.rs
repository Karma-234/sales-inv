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
   
     let cart_item = query_as::<_, CartItemModel>(CartSQLString::INSERT_CART_ITEM)
        .bind(&payload.cart_id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .bind(&payload.unit_amount)
        .fetch_optional(&mut *tx)
        .await.map_err(|e|MyBaseResponse::<()>::db_err(e))
        ;
    let cart_item_ok = cart_item.ok().unwrap();
    let item_str =  r#"SELECT quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2"#;
    let final_item_row = query_as::<_, CartItemModel>(item_str)
        .bind(payload.cart_id)
        .bind(payload.product_id)
        .fetch_one(&mut *tx)
        .await;
    let final_qty = match final_item_row {
        Ok(r) => r.quantity,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };
    let prev_qty_row = query_as::<_, CartItemModel>(
        r#"SELECT quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2"#,

    )
    .bind(payload.cart_id)
    .bind(payload.product_id)
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten();
    let prev_qty = prev_qty_row.map(|r| r.quantity).unwrap_or(0);
    let delta = final_qty - prev_qty;
    if delta > 0 {
        
        if let Err(e) = query_as::<_, ProductModel>(
            r#"UPDATE products SET quantity = quantity - $2, updated_at = now() WHERE id = $1"#,
        )
        .bind(payload.product_id)
        .bind(delta)
        .fetch_one(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    }
    if let Err(e) = tx.commit().await {
        return MyBaseResponse::db_err(e);
    }
    MyBaseResponse::ok(
    cart_item_ok,
    Some("Item updated in cart successfully".into())
)

    
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
    let mut tx = state.db.begin().await.unwrap();
    let existing_item = query_as::<_,CartItemModel>(
        r#"SELECT id, quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2 FOR UPDATE"#,
       
    ).bind(payload.cart_id)
    .bind(payload.product_id)
    .fetch_optional(&mut *tx)
    .await;
   let exs_res = match existing_item {
       
        Ok(r) => r,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };
    if exs_res.is_none() {
        let _ = tx.rollback().await;
            return MyBaseResponse::error(400, "Item not found in cart");
    }


   let old_qty = exs_res.as_ref().unwrap().quantity;
    let new_qty = payload.quantity;

    // If new quantity == 0 => remove item and restore stock
    if new_qty == 0 {
        // Lock product
        let product_lock = sqlx::query!(
            r#"SELECT id FROM products WHERE id = $1 FOR UPDATE"#,
            payload.product_id
        )
        .fetch_optional(&mut *tx)
        .await;
        if product_lock.is_err() || product_lock.as_ref().unwrap().is_none() {
            let _ = tx.rollback().await;
            return MyBaseResponse::error(404, "Product not found");
        }

        // Delete cart item
        if let Err(e) = sqlx::query!(
            r#"DELETE FROM cart_items WHERE cart_id = $1 AND product_id = $2"#,
            payload.cart_id,
            payload.product_id
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }

        // Restore stock
        if let Err(e) = sqlx::query!(
            r#"UPDATE products SET quantity = quantity + $2, updated_at = now() WHERE id = $1"#,
            payload.product_id,
            old_qty
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }

        if let Err(e) = tx.commit().await {
            return MyBaseResponse::db_err(e);
        }

        // Return a minimal response (deleted item)
        return MyBaseResponse::ok(None, Some("Item removed; stock restored".into()));
    }

    // Lock product for stock adjustments
    let product_row = match sqlx::query!(
        r#"SELECT id, quantity FROM products WHERE id = $1 FOR UPDATE"#,
        payload.product_id
    )
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };

    if product_row.is_none() {
        let _ = tx.rollback().await;
        return MyBaseResponse::error(404, "Product not found");
    }

    let available_stock = product_row.as_ref().unwrap().quantity;
    let delta = new_qty - old_qty;

    if delta > 0 && available_stock < delta {
        let _ = tx.rollback().await;
        return MyBaseResponse::error(400, "Insufficient product stock for increase");
    }

    // Update cart item
    let updated_item = match sqlx::query_as::<_, CartItemModel>(
        r#"
        UPDATE cart_items
        SET quantity = $3,
            unit_amount = $4,
            updated_at = now()
        WHERE cart_id = $1 AND product_id = $2
        RETURNING id, cart_id, product_id, quantity, unit_amount, line_total, created_at, updated_at
        "#
    )
    .bind(&payload.cart_id)
    .bind(&payload.product_id)
    .bind(&new_qty)
    .bind(&payload.unit_amount)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(ci) => ci,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };

    // Adjust product stock
    if delta != 0 {
        let stock_sql = if delta > 0 {
            // Reduce stock
            r#"UPDATE products SET quantity = quantity - $2, updated_at = now() WHERE id = $1"#
        } else {
            // Restore stock
            r#"UPDATE products SET quantity = quantity + $2, updated_at = now() WHERE id = $1"#
        };

        if let Err(e) = query_as::<_, ProductModel>(stock_sql)
            .bind(&payload.product_id)
            .bind(delta.abs())
            .fetch_optional(&mut *tx)
            .await
        {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    }

    if let Err(e) = tx.commit().await {
        return MyBaseResponse::db_err(e);
    }

    MyBaseResponse::ok(
        Some(updated_item),
        Some("Item updated in cart successfully".into()),
    )
}
