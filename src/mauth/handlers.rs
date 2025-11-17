use axum::{Json, extract::State};
use sqlx::query_as;

use crate::{
    AppState, mauth::schemas::LoginUserSchema, musers::models::MUserModel,
    shared_var::MyBaseResponse, util::passsword::compare_password,
};

pub async fn user_login_handler(
    State(app): State<AppState>,
    Json(payload): Json<LoginUserSchema>,
) -> MyBaseResponse<MUserModel> {
    let user_pass = payload.password;
    let user_email = payload.email;
    let get_user_sql = r#"
    SELECT * FROM users
    WHERE email = $1
    "#;
    let res = query_as::<_, MUserModel>(get_user_sql)
        .bind(user_email)
        .fetch_one(&app.db)
        .await;
    return match res {
        Ok(users) => {
            let verify = compare_password(&user_pass, &users.hashed_password);
            if verify {
                return MyBaseResponse::ok(Some(users), Some("Login successfull".into()));
            }
            return MyBaseResponse::error(401, format!("Invalid Credentials!",));
        }
        Err(e) => MyBaseResponse::error(401, format!("DB error: {}", e)),
    };
}
