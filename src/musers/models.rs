use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::Type, PartialEq, ToSchema)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "lowercase", type_name = "user_role")]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserRole::Admin => "Admin",
            UserRole::User => "User",
            UserRole::Guest => "Guest",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" | "admin" => Ok(UserRole::Admin),
            "User" | "user" => Ok(UserRole::User),
            "Guest" | "guest" => Ok(UserRole::Guest),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for UserRole {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        UserRole::from_str(&value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow, ToSchema, PartialEq)]
#[allow(non_snake_case)]
pub struct MUserModel {
    pub id: uuid::Uuid,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    pub role: UserRole,
    pub hashed_password: String,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub token_expiry: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
