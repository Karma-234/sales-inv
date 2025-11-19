use crate::AppState;
use crate::musers::models::MUserModel;
use crate::musers::schema::{AddUserSchema, DeleteUsersSchema, UpdateUsersSchema};
use crate::shared_var::{FilterOptions, MyBaseResponse};
use crate::util::passsword::hash_password;
use axum::Json;
use axum::extract::{Query, State};

use chrono::Utc;
use sqlx::query_as;

#[utoipa::path(
    get,
    path = "/api/v1/users/get", 
    tag = "Users",
    params(
        FilterOptions
    ),
    responses(
        (status = 200, description = "Users fetched successfully", body = MyBaseResponse<Vec<MUserModel>>),
        (status = 409, description = "Database error", body = MyBaseResponse<Vec<MUserModel>>),
    ),
     security(("bearerAuth" = [])), 
)]

pub async fn get_users_handler(
    State(app): State<AppState>,
    Query(opts): Query<FilterOptions>,
) -> MyBaseResponse<Vec<MUserModel>> {
    // Implementation for getting users with filtering, pagination, etc. based on FilterOptions
    if opts == FilterOptions::default() {
        // If no filter options provided, return all users
        let insert_sql = r#"
            SELECT *
            FROM users
            ORDER BY created_at DESC
            "#;
        let res = query_as::<_, MUserModel>(insert_sql)
            .fetch_all(&app.db)
            .await;
        return match res {
            Ok(user) => MyBaseResponse::ok(Some(user), Some("User created".into())),
            Err(e) => {
                eprintln!("database insert error: {}", e);
                MyBaseResponse::db_err(e)
            }
        };
    }
    if let Some(search_term) = opts.search {
        let pattern = format!("%{}%", search_term);
        let limit = opts.limit.unwrap_or(10);
        let offset = (opts.page.unwrap_or(1) - 1) * limit;

        let search_sql = r#"
                SELECT *,
                ts_rank(search_tsv, plainto_tsquery('english', $1)) AS rank
                FROM users
                WHERE search_tsv @@ plainto_tsquery('english', $1)
                ORDER BY rank DESC
                LIMIT $2 OFFSET $3;
                    "#;

        let res = query_as::<_, MUserModel>(search_sql)
            .bind(&pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&app.db)
            .await;

        return match res {
            Ok(users) => {
                println!("Fetched users: {:?}", users);
                MyBaseResponse::ok(Some(users), Some("Users fetched".into()))
            }
            Err(e) => {
                eprintln!("database query error: {}", e);
                MyBaseResponse::db_err(e)
            }
        };
    }
    let insert_sql = r#"
            SELECT *
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#;
    let res = query_as::<_, MUserModel>(insert_sql)
        .bind(opts.limit.unwrap_or(10))
        .bind((opts.page.unwrap_or(1) - 1) * opts.limit.unwrap_or(10))
        .fetch_all(&app.db)
        .await;
    return match res {
        Ok(user) => MyBaseResponse::ok(Some(user), Some("User created".into())),
        Err(e) => {
            eprintln!("database insert error: {}", e);
            MyBaseResponse::db_err(e)
        }
    };
}

#[utoipa::path(
    post,
    path = "/api/v1/users/create", 
    tag = "Users",
    request_body = AddUserSchema,
    responses(
        (status = 200, description = "User created successfully", body = MUserModel),
        (status = 409, description = "Database error"),
    ),
     security(("bearerAuth" = [])), 
)]
pub async fn create_new_user_handler(
    State(app): State<AppState>,
    Json(payload): Json<AddUserSchema>,
) -> MyBaseResponse<MUserModel> {
    // TODO: replace with a real password hashing function (bcrypt/argon2)
    let rehashed_password = hash_password(&payload.password);

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
            MyBaseResponse::db_err(e)
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/users/update", 
    tag = "Users",
    request_body = UpdateUsersSchema,
    responses(
        (status = 200, description = "User updated successfully", body = MUserModel),
        (status = 409, description = "Database error"),
    ),
     security(("bearerAuth" = [])), 
)]
pub async fn update_users_handler(
    State(app): State<AppState>,
    Json(payload): Json<UpdateUsersSchema>,
) -> MyBaseResponse<MUserModel> {
    let now = Utc::now();
    let rehashed_password: Option<String> = payload
        .hashed_password
        .as_ref()
        .and_then(|pw| hash_password(pw));

    let update_sql = r#"
        UPDATE users SET
            username = COALESCE($1, username),
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            email = COALESCE($4, email),
            role = COALESCE($5, role),
            hashed_password = COALESCE($6, hashed_password),
            is_verified = COALESCE($7, is_verified),
            verification_token = COALESCE($8, verification_token),
            token_expiry = COALESCE($9, token_expiry),
            updated_at = $10
        WHERE id = $11
        RETURNING id, username, first_name, last_name, email, role, hashed_password, created_at, updated_at,
            is_verified, verification_token, token_expiry
    "#;

    let res = query_as::<_, MUserModel>(update_sql)
        .bind(&payload.username)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.email)
        .bind(&payload.role)
        .bind(&rehashed_password)
        .bind(&payload.is_verified)
        .bind(&payload.verification_token)
        .bind(&payload.token_expiry)
        .bind(now)
        .bind(&payload.id)
        .fetch_one(&app.db)
        .await;

    match res {
        Ok(user) => MyBaseResponse::ok(Some(user), Some("User updated".into())),
        Err(e) => {
            eprintln!("database update error: {}", e);
            MyBaseResponse::db_err(e)
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/delete", 
    tag = "Users",
    request_body = DeleteUsersSchema,
    responses(
        (status = 200, description = "User deleted successfully", body = MUserModel),
        (status = 409, description = "Database error"),
    ),
     security(("bearerAuth" = [])), 
)]
pub async fn delete_users_handler(
    State(app): State<AppState>,
    Json(payload): Json<DeleteUsersSchema>,
) -> MyBaseResponse<MUserModel> {
    let delete_sql = r#"
        DELETE FROM users
        WHERE id = $1
        RETURNING *;
    "#;

    let res = query_as::<_, MUserModel>(delete_sql)
        .bind(&payload.id)
        .fetch_one(&app.db)
        .await;

    match res {
        Ok(datt) => MyBaseResponse::ok(Some(datt), Some("User deleted".into())),
        Err(e) => {
            eprintln!("database delete error: {}", e);
            return MyBaseResponse::db_err(e);
        }
    }
}
