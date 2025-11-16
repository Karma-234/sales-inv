use axum::Json;
use axum::extract::{Query, State};
// use
use sqlx::{Pool, Postgres, query_as};

use crate::AppState;
use crate::mproduct::models::ProductModel;
use crate::mproduct::schema::AddProductSchema;
use crate::shared_var::{FilterOptions, MyBaseResponse};

pub async fn get_product_handler(
    // opts: Option<Query<FilterOptions>>,
    Query(opts): Query<FilterOptions>,
    State(app_state): State<AppState>,
) -> MyBaseResponse<Vec<ProductModel>> {
    let limit = opts.limit.unwrap_or(10);

    let offset = (opts.page.unwrap_or(1) - 1) * limit;

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

    if let Err(e) = query_result {
        println!("database insertion error: {}", e);
    }

    MyBaseResponse::error(500, "Database query failed")

    // match query_result {
    //     Ok(p) => {
    //         println!("Fetched products after adding: {:?}", p);
    //         // MyBaseResponse::ok(Some(p), None)
    //         MyBaseResponse::ok(Some(p), Some("Product added successfully".into()))
    //     }
    //     Err(err) => {
    //         eprintln!("database query error: {}", err);
    //         MyBaseResponse::error(400, "Database query failed")
    //     }
    // }
}
pub async fn mock_post_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AddProductSchema>,
) -> MyBaseResponse<ProductModel> {
    let check_exists = query_as!(
        ProductModel,
        r#"
        SELECT *
        FROM products
        WHERE name = $1
        "#,
        payload.name,
    )
    .fetch_optional(&app_state.db)
    .await;
    if let Ok(Some(existing_product)) = check_exists {
        println!(
            "Product with name '{}' already exists: {:?}",
            payload.name, existing_product
        );
        return MyBaseResponse::error(409, "Product with the same name already exists");
    }
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
    .fetch_one(&app_state.db)
    .await;

    match query_result {
        Ok(p) => {
            println!("Fetched products: {:?}", p);
            MyBaseResponse::ok(Some(p), Some("Product added successfully".into()))
        }
        Err(err) => {
            eprintln!("database query error: {}", err);
            MyBaseResponse::error(500, "Database query failed")
        }
    }
}
