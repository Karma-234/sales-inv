use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate, ToSchema)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
