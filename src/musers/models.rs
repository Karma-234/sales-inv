use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromRow)]
#[allow(non_snake_case)]
pub struct MUserModel {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
