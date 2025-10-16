use axum::{Router, routing::get};
mod mproduct;
mod shared_var;

#[tokio::main]
async fn main() {
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
