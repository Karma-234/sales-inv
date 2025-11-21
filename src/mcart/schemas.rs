use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct AddCartItemSchema {
    pub cart_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub unit_amount: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct UpdateCartItemSchema {
    pub quantity: i32,
    pub product_id: uuid::Uuid,
    pub cart_id: uuid::Uuid,
    pub unit_amount: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct DeleteCartItemSchema {
    pub id: uuid::Uuid,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct CreateCartSchema {
    pub user_id: uuid::Uuid,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct UpdateCartStatusSchema {
    pub id: uuid::Uuid,
    pub status: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct ClearCartSchema {
    pub cart_id: uuid::Uuid,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct CheckoutCartSchema {
    pub cart_id: uuid::Uuid,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct GetCartByUserSchema {
    pub user_id: uuid::Uuid,
}
