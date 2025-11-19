use axum::Json;
use axum::extract::{Query, State};
// use
use sqlx::query_as;

use crate::AppState;
use crate::mproduct::models::ProductModel;
use crate::mproduct::schema::{AddProductSchema, DeleteProductSchema, UpdateProductSchema};
use crate::shared_var::{FilterOptions, MyBaseResponse};

#[utoipa::path(
    get,
    path = "/products/get", 
    tag = "Products",
    params(
        FilterOptions
    ),
    responses(
        (status = 200, description = "Products fetched successfully", body = [ProductModel]),
        (status = 500, description = "Database error"),
    )
)]
pub async fn get_product_handler(
    Query(opts): Query<FilterOptions>,
    State(app_state): State<AppState>,
) -> MyBaseResponse<Vec<ProductModel>> {
    let limit = opts.limit.unwrap_or(10);

    let _offset = (opts.page.unwrap_or(1) - 1) * limit;

    if let Some(search_term) = opts.search {
        println!("Searching for products with term: {}", search_term);
        let search_result = query_as!(
            ProductModel,
            r#"
            SELECT *
            FROM products
            WHERE name ILIKE $1
            ORDER BY created_at DESC
            "#,
            format!("%{}%", search_term.to_string()),
        )
        .fetch_all(&app_state.db)
        .await;
        match search_result {
            Ok(p) => {
                println!("Fetched products: {:?}", p);
                return MyBaseResponse::ok(Some(p), Some("Product retrieved successfully".into()));
            }
            Err(err) => {
                eprintln!("database query error: {}", err);
                return MyBaseResponse::error(500, "Database query failed");
            }
        }
    }

    let query_result = query_as!(
        ProductModel,
        r#"
        SELECT *
        FROM products ORDER by created_at DESC

        "#,
    )
    .fetch_all(&app_state.db)
    .await;

    match query_result {
        Ok(p) => {
            println!("Fetched products: {:?}", p);
            MyBaseResponse::ok(Some(p), Some("Product retrieved successfully".into()))
        }
        Err(err) => {
            eprintln!("database query error: {}", err);
            MyBaseResponse::error(500, "Database query failed")
        }
    }
}

#[utoipa::path(
    post,
    path = "/products/add", 
    tag = "Products",
    request_body = AddProductSchema,
    responses(
        (status = 200, description = "Product added successfully", body = ProductModel),
        (status = 500, description = "Database error"),
    )
)]
pub async fn add_product_handler(
    Json(payload): Json<AddProductSchema>,
    State(app): State<AppState>,
) -> MyBaseResponse<ProductModel> {
    println!("Request received to add product: {:?}", payload);

    let query_result = query_as!(
        ProductModel,
        r#"
        INSERT INTO products (id, name, price, quantity, pack_price, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        uuid::Uuid::new_v4(),
        payload.name,
        payload.price,
        payload.quantity as i32,
        payload.pack_price,
        payload.created_at.unwrap_or_else(chrono::Utc::now),
        payload.updated_at.unwrap_or_else(chrono::Utc::now),
    )
    .fetch_one(&app.db)
    .await;
    match query_result {
        Ok(p) => {
            println!("Fetched products: {:?}", p);
            return MyBaseResponse::ok(Some(p), Some("Product added successfully".into()));
        }
        Err(err) => {
            eprintln!("database query error: {}", err);
            MyBaseResponse::db_err(err)
        }
    }
}

#[utoipa::path(
    put,
    path = "/products/update", 
    tag = "Products",
    request_body = UpdateProductSchema,
    responses(
        (status = 200, description = "Product updated successfully", body = ProductModel),
        (status = 409, description = "Product not found"),
        (status = 500, description = "Database error"),
    )
)]
pub async fn update_prod_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UpdateProductSchema>,
) -> MyBaseResponse<ProductModel> {
    let check_exists = query_as!(
        ProductModel,
        r#"
        SELECT *
        FROM products
        WHERE id = $1
        "#,
        payload.id,
    )
    .fetch_optional(&app_state.db)
    .await;
    if let Ok(Some(existing_product)) = check_exists {
        let updated_prod = ProductModel {
            id: existing_product.id,
            name: payload.name.clone().unwrap_or(existing_product.name),
            price: payload.price.unwrap_or(existing_product.price),
            quantity: payload.quantity.unwrap_or(existing_product.quantity),
            pack_price: payload.pack_price.or(existing_product.pack_price),
            created_at: existing_product.created_at,
            updated_at: Some(chrono::Utc::now()),
        };
        let query_result = query_as!(
            ProductModel,
            r#"
            UPDATE products
            SET name = $1, price = $2, quantity = $3, pack_price = $4, updated_at = $5
            WHERE id = $6
            RETURNING *
            "#,
            updated_prod.name,
            updated_prod.price,
            updated_prod.quantity,
            updated_prod.pack_price,
            updated_prod.updated_at,
            updated_prod.id,
        )
        .fetch_one(&app_state.db)
        .await;
        match query_result {
            Ok(p) => {
                println!("Fetched products: {:?}", p);
                return MyBaseResponse::ok(Some(p), Some("Product updated successfully".into()));
            }
            Err(err) => {
                eprintln!("database query error: {}", err);
                return MyBaseResponse::db_err(err);
            }
        }
    }

    return MyBaseResponse::error(409, "Product not found!");
}

#[utoipa::path(
    delete,
    path = "/products/delete", 
    tag = "Products",
    request_body = DeleteProductSchema,
    responses(
        (status = 200, description = "Product deleted successfully", body = ProductModel),
        (status = 500, description = "Database error"),
    )
)]
pub async fn del_prod_handler(
    Json(payload): Json<DeleteProductSchema>,
    State(app): State<AppState>,
) -> MyBaseResponse<ProductModel> {
    println!("Request received to delete product: {:?}", payload);

    let query_result = query_as!(
        ProductModel,
        r#"
        DELETE FROM products
        WHERE id = $1
        RETURNING *
        "#,
        payload.id,
    )
    .fetch_one(&app.db)
    .await;

    match query_result {
        Ok(p) => {
            println!("Fetched products: {:?}", p);
            return MyBaseResponse::ok(Some(p), Some("Product deleted successfully".into()));
        }
        Err(err) => {
            eprintln!("database query error: {}", err);
            return MyBaseResponse::db_err(err);
        }
    }
}
