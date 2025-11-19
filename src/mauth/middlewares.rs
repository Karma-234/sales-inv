use crate::musers::models::UserRole;
use crate::util::token::has_token_expired;
use crate::{
    AppState, musers::models::MUserModel, shared_var::MyBaseResponse, util::token::decode_token,
};
use axum::body::Body;
use axum::http::header;
use axum::response::IntoResponse;
use axum::{extract::Request, response::Response};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: MUserModel,
}

pub async fn auth_middleware(
    mut request: Request,
    // next: Next,
    state: AppState,
) -> Result<Request<Body>, Response> {
    // try Authorization: Bearer <token>
    let app = state;
    let token_opt = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_owned()))
        // fallback: parse Cookie header manually for token=...
        .or_else(|| {
            request
                .headers()
                .get(header::COOKIE)
                .and_then(|c| c.to_str().ok())
                .and_then(|cookie_str| {
                    cookie_str
                        .split(';')
                        .map(|s| s.trim())
                        .find_map(|pair| pair.strip_prefix("token=").map(|v| v.to_string()))
                })
        });

    let token = match token_opt {
        Some(t) => t,
        None => return Err(MyBaseResponse::<()>::error(401, "Unauthorised!").into_response()),
    };

    let user_id_str = match decode_token(&token, app.env.jwt_secret.as_bytes()) {
        Ok(s) => s,
        Err(_) => return Err(MyBaseResponse::<()>::error(401, "Unauthorised!").into_response()),
    };

    let user_id = match uuid::Uuid::parse_str(user_id_str.trim()) {
        Ok(id) => id,
        Err(_) => {
            return Err(MyBaseResponse::<()>::error(401, "Invalid credentials!").into_response());
        }
    };

    let query_text = r#"
        SELECT *
        FROM users
        WHERE id = $1
    "#;

    let user_res = query_as::<_, MUserModel>(query_text)
        .bind(user_id)
        .fetch_one(&app.db)
        .await;

    let user = match user_res {
        Ok(u) => u,
        Err(_) => {
            return Err(MyBaseResponse::<()>::error(401, "User does not exist!").into_response());
        }
    };
    if has_token_expired(user.token_expiry.unwrap().timestamp()) {
        return Err(MyBaseResponse::<()>::error(401, "Session expired!").into_response());
    }

    request
        .extensions_mut()
        .insert(JWTAuthMiddleware { user: user.clone() });

    let req_user = request
        .extensions()
        .get::<JWTAuthMiddleware>()
        .unwrap()
        .user
        .clone();
    println!("Authenticated User in Middleware: {:?}", req_user);

    Ok(request)
}

pub async fn auth_middleware_with_admin_perms(request: Request) -> Result<Request<Body>, Response> {
    let req_user = request.extensions().get::<JWTAuthMiddleware>();
    if req_user.is_none() {
        return Err(MyBaseResponse::<()>::error(401, "Unauthorised!").into_response());
    }
    let user = &req_user.unwrap().user;

    if user.role != UserRole::Admin {
        return Err(MyBaseResponse::<()>::error(403, "Forbidden!").into_response());
    }
    Ok(request)
}
