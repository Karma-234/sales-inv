use crate::AppState;
use crate::mauth::middlewares::JWTAuthMiddleware;
use crate::mcart::models::{CartItemModel, CartModel, CartWithItemsModel};
use crate::mcart::schemas::{
    AddCartItemSchema, UpdateCartItemSchema,
};
use crate::mcart::sql_string::CartSQLString;
use crate::shared_var::MyBaseResponse;

use axum::body::Body;
use axum::extract::Request;
use sqlx::query_as;

#[utoipa::path(
    post,
    path = "/api/v1/cart/create", 
    tag = "Carts",
    responses(
        (status = 200, description = "Carts created successfully", body = MyBaseResponse<Vec<CartModel>>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartModel>),
    ),
     security(("bearerAuth" = [])), 
)]
pub async fn create_cart_handler(
    request: Request<Body>,
    state: AppState,
) -> MyBaseResponse<CartModel> {
    let req_user = request.extensions().get::<JWTAuthMiddleware>();
    if req_user.is_none() {
        return MyBaseResponse::<CartModel>::error(400,"Unauthorised!");
    }
    let user = &req_user.unwrap().user;
    let res = query_as::<_, CartModel>(CartSQLString::CREATE_CART_ID)
        .bind(&user.id)
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
    responses(
        (status = 200, description = "Carts retrieved successfully", body = MyBaseResponse<Vec<CartWithItemsModel>>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartWithItemsModel>),
    )
     
)]
pub async fn get_cart_by_user_handler(
    request: Request<Body>,
    state: AppState,
) -> MyBaseResponse<Vec<CartWithItemsModel>> {
      let req_user = request.extensions().get::<JWTAuthMiddleware>();
    if req_user.is_none() {
        return MyBaseResponse::<Vec<CartWithItemsModel>>::error(400,"Unauthorised!");
    }
    let user = &req_user.unwrap().user;
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_CART_BY_USER_ID)
        .bind(&user.id)
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
    responses(
        (status = 200, description = "Open cart retrieved successfully", body = MyBaseResponse<CartWithItemsModel>),
        (status = 409, description = "Database error", body = MyBaseResponse<CartWithItemsModel>),
    )
     
)]
pub async fn get_open_cart_by_user_handler(
    request: Request<Body>,
    
    state: AppState,
) -> MyBaseResponse<CartWithItemsModel> {
      let req_user = request.extensions().get::<JWTAuthMiddleware>();
      println!("USER JWT{:?}", req_user);
    if req_user.is_none() {
        return MyBaseResponse::<CartWithItemsModel>::error(400,"Unauthorised!");
    }
    let user = &req_user.unwrap().user;
    let res = query_as::<_, CartWithItemsModel>(CartSQLString::GET_OPEN_CART_BY_USER_ID)
        .bind(&user.id)
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
        return MyBaseResponse::error(400, "Quantity must be > 0");
    }

       match sqlx::query!(
        r#"SELECT quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2"#,
        payload.cart_id,
        payload.product_id
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(row)) => {
            
            let new_qty = row.quantity + payload.quantity;
            let update_payload = UpdateCartItemSchema {
                cart_id: payload.cart_id,
                product_id: payload.product_id,
                quantity: new_qty,
                unit_amount: payload.unit_amount,
            };
            return update_item_in_cart_handler(
                axum::extract::Json(update_payload),
                state,
            ).await;
        }
        Ok(None) => { 
            
            let mut tx = match state.db.begin().await {
                Ok(t) => t,
                Err(e) => return MyBaseResponse::db_err(e),
            };

            
            let dec = sqlx::query!(
                r#"UPDATE products
                   SET quantity = quantity - $2, updated_at = now()
                   WHERE id = $1 AND quantity >= $2
                   RETURNING id"#,
                payload.product_id,
                payload.quantity
            )
            .fetch_optional(&mut *tx)
            .await;

            match dec {
                Ok(Some(_)) => {}
                Ok(None) => {
                    let _ = tx.rollback().await;
                    return MyBaseResponse::error(400, "Insufficient stock");
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    return MyBaseResponse::db_err(e);
                }
            }

            
            let inserted = sqlx::query_as::<_, CartItemModel>(
                r#"INSERT INTO cart_items (cart_id, product_id, quantity, unit_amount, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, now(), now())
                   RETURNING id, cart_id, product_id, quantity, unit_amount, line_total, created_at, updated_at"#
            )
            .bind(&payload.cart_id)
            .bind(&payload.product_id)
            .bind(&payload.quantity)
            .bind(&payload.unit_amount)
            .bind(payload.quantity as f64 * payload.unit_amount)
            .fetch_one(&mut *tx)
            .await;

            let inserted = match inserted {
                Ok(ci) => ci,
                Err(e) => {
                    let _ = tx.rollback().await;
                    return MyBaseResponse::db_err(e);
                }
            };

            if let Err(e) = tx.commit().await {
                return MyBaseResponse::db_err(e);
            }

            return MyBaseResponse::ok(Some(inserted), Some("Item added to cart".into()));
         }
        Err(e) => return MyBaseResponse::db_err(e),
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
    ;

    let mut tx = match state.db.begin().await {
        Ok(t) => t,
        Err(e) => return MyBaseResponse::db_err(e),
    };

    
    let existing = sqlx::query!(
        r#"SELECT quantity FROM cart_items
           WHERE cart_id = $1 AND product_id = $2
           FOR UPDATE"#,
        payload.cart_id,
        payload.product_id
    )
    .fetch_optional(&mut *tx)
    .await;

    let old_qty = match existing {
        Ok(Some(r)) => r.quantity,
        Ok(None) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::error(404, "Item not found");
        }
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };

    let new_qty = payload.quantity;
    let delta = new_qty - old_qty;

    if new_qty == 0 {
        
        let del = sqlx::query!(
            r#"DELETE FROM cart_items
               WHERE cart_id = $1 AND product_id = $2
               RETURNING *"#,
            payload.cart_id,
            payload.product_id,
            
        )
        .fetch_one(&mut *tx)
        .await;

        if del.is_err() {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(del.err().unwrap());
        }

        let restore = sqlx::query!(
            r#"UPDATE products SET quantity = quantity + $2, updated_at = now()
               WHERE id = $1"#,
            payload.product_id,
            old_qty
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = restore {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }

        if let Err(e) = tx.commit().await {
            return MyBaseResponse::db_err(e);
        }

        return MyBaseResponse::ok(None, Some("Item removed".into()));
    }

    
    if delta > 0 {
        
        let dec = sqlx::query!(
            r#"UPDATE products
               SET quantity = quantity - $2, updated_at = now()
               WHERE id = $1 AND quantity >= $2
               RETURNING id"#,
            payload.product_id,
            delta
        )
        .fetch_optional(&mut *tx)
        .await;

        match dec {
            Ok(Some(_)) => {}
            Ok(None) => {
                let _ = tx.rollback().await;
                return MyBaseResponse::error(400, "Insufficient stock for increase");
            }
            Err(e) => {
                let _ = tx.rollback().await;
                return MyBaseResponse::db_err(e);
            }
        }
    } else if delta < 0 {
        
        let inc = sqlx::query!(
            r#"UPDATE products
               SET quantity = quantity + $2, updated_at = now()
               WHERE id = $1"#,
            payload.product_id,
            (-delta)
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = inc {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    }

    
    let updated = sqlx::query_as::<_, CartItemModel>(
        r#"UPDATE cart_items
           SET quantity = $3,
               unit_amount = $4,
               updated_at = now()
           WHERE cart_id = $1 AND product_id = $2
           RETURNING id, cart_id, product_id, quantity, unit_amount, line_total, created_at, updated_at"#
    )
    .bind(&payload.cart_id)
    .bind(&payload.product_id)
    .bind(new_qty)
    .bind(&payload.unit_amount)
    .fetch_one(&mut *tx)
    .await;

    let updated = match updated {
        Ok(ci) => ci,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);
        }
    };

    if let Err(e) = tx.commit().await {
        return MyBaseResponse::db_err(e);
    }

    MyBaseResponse::ok(Some(updated), Some("Item updated".into()))
}

pub async fn remove_items_from_cart_handler(
     payload: axum::extract::Json<Vec<UpdateCartItemSchema>>,
    state: AppState
) -> MyBaseResponse<Vec<CartItemModel>> {
    // Implementation for removing multiple items from cart --- IGNORE ---
    if payload.0.is_empty() {
        return MyBaseResponse::error(400, "No items to remove!");
    }
    let items = payload.0;
    let cart_id = items[0].cart_id;
    if let Some(false) = items.iter().find(|item| item.cart_id != cart_id).map(|_| false) {
        return MyBaseResponse::error(400, "All items must belong to the same cart!");
    }
    let mut sorted_items = items;
    sorted_items.sort_by(|a, b| a.product_id.cmp(&b.product_id));
    let mut product_ids: Vec<uuid::Uuid> = sorted_items.iter().map(|item| item.product_id).collect();
    let mut tx = match state.db.begin().await {
        Ok(t) => t,
        Err(e) => return MyBaseResponse::db_err(e),
    };
    let check_cart_status = sqlx::query_scalar!(
        r#"SELECT status::TEXT FROM carts WHERE id = $1 FOR UPDATE"#,
        cart_id
    )
    .fetch_one(&mut *tx)
    .await;
    let status = match check_cart_status {
        Ok(s) => s,
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);  
        }
    };
    if status.unwrap() != "open" {
        let _ = tx.rollback().await;
        return MyBaseResponse::error(400, "Cannot modify a closed cart!");
    }
    let existin_items = sqlx::query!(
        r#"SELECT product_id, quantity FROM cart_items
           WHERE cart_id = $1 AND product_id = ANY($2)
           FOR UPDATE"#,
        cart_id,
        &product_ids
    )
    .fetch_all(&mut *tx)
    .await;
    let existing_map = match existin_items {
        Ok(rows) => {
            let mut map = std::collections::HashMap::new();
            for row in rows {
                map.insert(row.product_id, row.quantity);
            }
            map
        },
        Err(e) => {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);  
        }
    };
   for map in existing_map.iter() {
        let product_id = map.0;
        let qty = map.1;
        let dec = sqlx::query!(
            r#"UPDATE products
               SET quantity = quantity + $2, updated_at = now()
               WHERE id = $1"#,
            product_id,
            qty
        )
        .execute(&mut *tx)
        .await;
        if let Err(e) = dec {
            let _ = tx.rollback().await;
            return MyBaseResponse::db_err(e);  
        }
    }
    let del = sqlx::query!(
        r#"DELETE FROM cart_items
           WHERE cart_id = $1 AND product_id = ANY($2)"#,
        cart_id,
        &product_ids
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = del {
        let _ = tx.rollback().await;
        return MyBaseResponse::db_err(e);  
    }
    if let Err(e) = tx.commit().await {
        return MyBaseResponse::db_err(e);  
    }

    MyBaseResponse::ok(None, Some("Items removed".into()))
}