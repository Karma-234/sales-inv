use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use sqlx::query_as;

use crate::{
    AppState,
    mauth::schemas::LoginUserSchema,
    musers::models::MUserModel,
    shared_var::MyBaseResponse,
    util::{passsword::compare_password, token::create_token},
};

#[utoipa::path(
    post,
    path = "/api/v1/auth/login", 
    tag = "Authentication",
    request_body = LoginUserSchema,
    responses(
        (status = 200, description = "User logged in successfully", body = MyBaseResponse<MUserModel>),
        (status = 401, description = "Unauthorized", body = MyBaseResponse<MUserModel>),
    )
)]
pub async fn user_login_handler(
    State(app): State<AppState>,
    Json(payload): Json<LoginUserSchema>,
) -> MyBaseResponse<MUserModel> {
    let user_pass = payload.password;
    let user_email = payload.email;
    let now = Utc::now();
    let get_user_sql = r#"
    SELECT * FROM users
    WHERE email = $1
    "#;
    let res = query_as::<_, MUserModel>(get_user_sql)
        .bind(user_email)
        .fetch_one(&app.db)
        .await;
    match res {
        Ok(users) => {
            let verify = compare_password(&user_pass, &users.hashed_password);

            if verify == true {
                let token_expiry = now + Duration::minutes(30);
                let token = create_token(
                    &users.id.to_string(),
                    &app.env.jwt_secret.as_bytes(),
                    token_expiry.timestamp(),
                );
                match token {
                    Ok(token_detail) => {
                        println!("Gemerated Token: {:?}", token_detail);

                        let update_token_sql = r#"
                            UPDATE users
                            SET
                            is_verified = true,
                            verification_token = COALESCE($2, verification_token),
                            token_expiry = COALESCE($3, token_expiry),
                            updated_at = COALESCE($4, updated_at)
                            WHERE id = $1
                            RETURNING *
                            "#;
                        let res = query_as::<_, MUserModel>(update_token_sql)
                            .bind(&users.id)
                            .bind(token_detail)
                            .bind(token_expiry)
                            .bind(now)
                            .fetch_one(&app.db)
                            .await;

                        return match res {
                            Ok(data) => {
                                MyBaseResponse::ok(Some(data), Some(format!("Login Succesful!",)))
                            }
                            Err(_) => MyBaseResponse::error(401, "Could not authorize!"),
                        };
                    }

                    Err(_) => {
                        return MyBaseResponse::error(401, format!("Could not create JWT!",));
                    }
                }
            }
            return MyBaseResponse::error(401, format!("Invalid Credentials!"));
        }

        Err(e) => {
            return MyBaseResponse::error(401, format!("DB error: {}", e));
        }
    };
}
