#[derive(serde::Serialize, Debug, Clone)]
pub struct ProductModel {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub pack_price: f64,
}
