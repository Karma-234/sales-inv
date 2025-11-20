pub struct AddCartItemSchema {
    pub cart_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub quantity: i32,
}
pub struct UpdateCartItemSchema {
    pub id: uuid::Uuid,
    pub quantity: i32,
}
pub struct DeleteCartItemSchema {
    pub id: uuid::Uuid,
}
pub struct CreateCartSchema {
    pub user_id: uuid::Uuid,
}
pub struct UpdateCartStatusSchema {
    pub id: uuid::Uuid,
    pub status: String,
}
pub struct ClearCartSchema {
    pub cart_id: uuid::Uuid,
}
pub struct CheckoutCartSchema {
    pub cart_id: uuid::Uuid,
}
pub struct GetCartByUserSchema {
    pub user_id: uuid::Uuid,
}
