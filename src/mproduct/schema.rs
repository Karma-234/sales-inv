use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddProductSchema {
    #[serde()]
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    #[serde(rename = "packPrice")]
    pub pack_price: Option<f64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpdateProductSchema {
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
    #[serde()]
    pub id: uuid::Uuid,
}
