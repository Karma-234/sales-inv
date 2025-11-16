use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tower_http::cors::CorsLayer;

use crate::{
    mproduct::models::ProductModel,
    shared_var::{MyBaseResponse, create_router},
};
mod mproduct;
mod shared_var;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let address = "127.0.0.1:7777";
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create Postgres connection pool");
    println!("Hello, world!");

    let app_cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::DELETE, Method::POST, Method::PUT])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(address.parse::<HeaderValue>().unwrap());

    let app = create_router(AppState {
        db: db_pool.clone(),
    })
    .layer(app_cors);

    // Define IP and listener

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Serve axum app
    println!("Listening on http://{}", address);
    let _ = axum::serve(listener, app).await;
}
