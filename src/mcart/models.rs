use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::Type, PartialEq, ToSchema)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "lowercase", type_name = "cart_status")]
pub enum CartStatus {
    Open,
    Refund,
    Paid,
    FOC,
}

impl fmt::Display for CartStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CartStatus::Open => "Open",
            CartStatus::Refund => "Refund",
            CartStatus::Paid => "Paid",
            CartStatus::FOC => "FOC",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for CartStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open" | "open" => Ok(CartStatus::Open),
            "Refund" | "refund" => Ok(CartStatus::Refund),
            "Paid" | "paid" => Ok(CartStatus::Paid),
            "FOC" | "foc" => Ok(CartStatus::FOC),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for CartStatus {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        CartStatus::from_str(&value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct CartModel {
    pub id: uuid::Uuid,
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    pub status: CartStatus,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct CartItemModel {
    pub id: uuid::Uuid,
    #[serde(rename = "cartId")]
    pub cart_id: uuid::Uuid,
    #[serde(rename = "productId")]
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct CartItemWithProductModel {
    pub id: uuid::Uuid,
    #[serde(rename = "cartId")]
    pub cart_id: uuid::Uuid,
    #[serde(rename = "productId")]
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub product: Box<super::mproduct::models::ProductModel>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct CartWithItemsModel {
    pub id: uuid::Uuid,
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    pub status: CartStatus,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    pub items: Vec<CartItemWithProductModel>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
impl CartWithItemsModel {
    pub fn new(cart: CartModel, items: Vec<CartItemWithProductModel>) -> Self {
        Self {
            id: cart.id,
            user_id: cart.user_id,
            status: cart.status,
            total_amount: cart.total_amount,
            items,
            created_at: cart.created_at,
            updated_at: cart.updated_at,
        }
    }
}
