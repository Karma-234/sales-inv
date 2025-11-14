#[derive(serde::Serialize, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct ProductModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub pack_price: f64,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
}
