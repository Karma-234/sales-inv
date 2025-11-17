use crate::AppState;
use crate::musers::models::{MUserModel, UserRole};
use crate::musers::schema::AddUserSchema;
use crate::shared_var::MyBaseResponse;
use axum::Json;
use axum::extract::{Query, State};

use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

/// Insert a new user into the `users` table and return the created row.
/// Expects AddUserSchema to provide: username, first_name, last_name, email, password (or hashed_password), role (optional).
pub async fn create_new_user_handler(
    State(app): State<AppState>,
    Json(payload): Json<AddUserSchema>,
) -> MyBaseResponse<MUserModel> {
    // TODO: replace with a real password hashing function (bcrypt/argon2)
    let rehashed_password = &payload.password;

    let now = Utc::now();

    let role = payload.role.clone();

    let insert_sql = r#"
        INSERT INTO users (
             username, first_name, last_name, email, role, hashed_password, created_at, updated_at, is_verified, verification_token, token_expiry
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, username, first_name, last_name, email, role, hashed_password, created_at, updated_at,
            is_verified, verification_token, token_expiry
    "#;

    let res = query_as::<_, MUserModel>(insert_sql)
        .bind(&payload.username)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.email)
        .bind(role) // requires sqlx mapping for UserRole (you have sqlx::Type derive)
        .bind(&rehashed_password)
        .bind(now)
        .bind(now)
        .bind(&payload.is_verified)
        .bind(&payload.verification_token)
        .bind(&payload.token_expiry)
        .fetch_one(&app.db)
        .await;

    match res {
        Ok(user) => MyBaseResponse::ok(Some(user), Some("User created".into())),
        Err(e) => {
            eprintln!("database insert error: {}", e);
            MyBaseResponse::error(409, format!("DB error: {}", e))
        }
    }
}

pub async fn get_user_handler() {}

pub async fn update_user_handler() {}
pub async fn delete_user_handler() {}
