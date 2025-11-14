use sqlx::prelude::FromRow;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow)]
#[allow(non_snake_case)]
pub struct ProductModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    #[serde(rename = "packPrice")]
    pub pack_price: f64,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
}
