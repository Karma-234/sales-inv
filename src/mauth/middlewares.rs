use crate::{
    AppState, musers::models::MUserModel, shared_var::MyBaseResponse, util::token::decode_token,
};
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
};
use axum_extra::extract::CookieJar;
use sqlx::query_as;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddeware {
    pub user: MUserModel,
}
pub async fn auth_middleware(
    request: Request,
    next: Next,
    cookie_jar: CookieJar,
    State(app): State<AppState>,
) {
    let cookies = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| {
                    auth_header.to_str().ok().and_then(|auth_value| {
                        if auth_value.starts_with("Bearer ") {
                            Some(auth_value[7..].to_owned())
                        } else {
                            None
                        }
                    })
                })
        });
    let token = cookies
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string()))?;
    let token_details = match decode_token(token, app.env.jwt_secret.as_bytes()) {
        Ok(e) => e,
        Err(e) => MyBaseResponse::error(401, "Unauthorised!"),
    };

    let user_id = uuid::Uuid::parse_str(&token_details.to_string())
        .map_err(|_| MyBaseResponse::error(401, "Invalid credentials!"));

    let query_text = r#"
    SELEECT * FROM users 
    WHERE id = $1"#;
    let res = query_as::<_, MUserModel>(query_text)
        .bind(&user_id)
        .fetch_one(&app.db)
        .await
        .map_err(|_| MyBaseResponse::error(401, "User does not exist!"));

    let user = res
        .ok()
        .or_else(|| MyBaseResponse::error(401, "User does not exist!"));
    request
        .extensions_mut()
        .insert(JWTAuthMiddeware { user: user.clone() });

    Ok(next.run(request).await)
}
