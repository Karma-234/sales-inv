use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::Type, PartialEq, ToSchema)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "lowercase", type_name = "cart_status")]
pub enum CartStatus {
    Admin,
    User,
    Guest,
}

impl fmt::Display for CartStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CartStatus::Admin => "Admin",
            CartStatus::User => "User",
            CartStatus::Guest => "Guest",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for CartStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" | "admin" => Ok(CartStatus::Admin),
            "User" | "user" => Ok(CartStatus::User),
            "Guest" | "guest" => Ok(CartStatus::Guest),
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
