use crate::mproduct::models::ProductModel;
use crate::shared_var::MyBaseResponse;

pub async fn get_product_handler() -> MyBaseResponse<ProductModel> {
    // Build a sample product. Replace with DB lookup + error handling.
    let product = ProductModel {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Sample Widget".to_string(),
        price: 19.99,
        quantity: 42,
        pack_price: 0.0,
    };

    MyBaseResponse::ok(Some(product))
}
