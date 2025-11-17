use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddProductSchema {
    #[validate(length(min = 1), message = "ID cannot be empty")]
    #[serde()]
    pub id: Option<uuid::Uuid>,
    #[validate(length(min = 1), message = "Name cannot be empty")]
    pub name: String,
    #[validate(length(min = 1), message = "Price cannot be empty")]
    #[validate(range(min = 0.0), message = "Price must be non-negative")]
    pub price: f64,
    pub quantity: u32,
    #[serde(rename = "packPrice")]
    pub pack_price: Option<f64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate)]
pub struct UpdateProductSchema {
    #[validate(length(min = 1))]
    #[serde()]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_price: Option<f64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeleteProductSchema {
    #[validate(length(min = 1), message = "ID cannot be empty")]
    #[serde()]
    pub id: uuid::Uuid,
}
