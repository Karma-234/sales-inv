use axum::extract::Query;
// use sqlx;
use sqlx::{Pool, Postgres, query_as};

use crate::AppState;
use crate::mproduct::models::ProductModel;
use crate::shared_var::{FilterOptions, MyBaseResponse};

pub async fn get_product_handler(
    opts: Option<Query<FilterOptions>>,
    app_state: AppState,
) -> MyBaseResponse<ProductModel> {
    // Build a sample product. Replace with DB lookup + error handling.

    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);

    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = query_as!(
        ProductModel,
        r#"
        SELECT *
        FROM products ORDER by id
        LIMIT $1 OFFSET $2
        "#,
        limit as i64,
        offset as i64
    )
    .fetch_all(&app_state.db)
    .await;
    if query_result.is_err() {
        MyBaseResponse::error(500, "Database query failed")
    } else {
        let product = ProductModel {
            id: uuid::Uuid::new_v4(),
            name: "Sample Widget".to_string(),
            price: 19.99,
            quantity: 42,
            pack_price: 0.0,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        MyBaseResponse::ok(Some(product))
    }
}
