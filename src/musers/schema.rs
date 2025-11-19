use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use validator::Validate;

use crate::musers::models::UserRole;

#[derive(serde::Serialize, serde::Deserialize, Debug, Validate, ToSchema, PartialEq)]
pub struct AddUserSchema {
    #[serde()]
    pub id: Option<uuid::Uuid>,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    pub role: UserRole,
    pub password: String,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub token_expiry: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema, PartialEq)]
pub struct UpdateUsersSchema {
    #[serde()]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_expiry: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema, PartialEq)]
pub struct DeleteUsersSchema {
    #[serde()]
    pub id: uuid::Uuid,
}
