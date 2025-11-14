#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddProductSchema {
    #[serde()]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UpdateProductSchema {
    #[serde()]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_price: Option<f64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeleteProductSchema {
    #[serde()]
    pub id: uuid::Uuid,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::NaiveDateTime,
}
