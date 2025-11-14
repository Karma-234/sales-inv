use axum::{Router, routing::get};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
mod mproduct;
mod shared_var;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create Postgres connection pool");
    println!("Hello, world!");

    let app = Router::new().route(
        &mproduct::routes::get_products(),
        get(mproduct::api::get_product_handler),
    );

    // Define IP and listener
    let address = "127.0.0.1:7777";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Serve axum app
    println!("Listening on http://{}", address);
    let _ = axum::serve(listener, app).await;
}
