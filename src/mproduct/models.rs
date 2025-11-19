use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct ProductModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    #[serde(rename = "packPrice")]
    pub pack_price: Option<f64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
