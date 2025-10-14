use axum::Router;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new();

    // Define IP and listener
    let address = "0.0.0.0:7777";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Serve axum app
    axum::serve(listener, app);
}
