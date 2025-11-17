use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
