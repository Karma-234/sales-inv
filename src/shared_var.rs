use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use axum::{Json, Router};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::{AppState, mauth, mproduct, musers};

#[derive(serde::Serialize, Debug, Clone, ToSchema)]
#[serde(bound = "T: serde::Serialize")]
pub struct MyBaseResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> MyBaseResponse<T> {
    /// Convenience constructor for successful responses.
    pub fn ok(data: Option<T>, message: Option<String>) -> Self {
        Self {
            code: 200,
            message: message.unwrap_or("OK".into()),
            data,
        }
    }

    /// Convenience constructor for error responses.
    pub fn error(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
        pub fn db_err(e: sqlx::Error) -> MyBaseResponse<Vec<FieldError>> {
        use sqlx::error::DatabaseError;
        match e {
            sqlx::Error::RowNotFound => MyBaseResponse {
                code: 404,
                message: "Record not found".into(),
                data: Some(vec![FieldError::new("", "Record not found", "ROW_NOT_FOUND")]),
            },
            sqlx::Error::Database(db) => {
                let fe_list = map_pg_database_error(db.as_ref());
                let code = if fe_list.iter().any(|f| f.code == "23505") { 409 } else { 400 };
                MyBaseResponse {
                    code,
                    message: fe_list.get(0).map(|f| f.message.clone()).unwrap_or("Database constraint error".into()),
                    data: Some(fe_list),
                }
            }
            _ => MyBaseResponse {
                code: 500,
                message: format!("Database error: {}", e),
                data: Some(vec![FieldError::new("", e.to_string(), "DB_ERROR")]),
            },
        }
    }
}

impl<T> IntoResponse for MyBaseResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        // Map `code` (u32) to an HTTP status if possible, otherwise default to 200
        let status = StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::OK);
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, ToSchema)]
pub struct FilterOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(

        mauth::handlers::user_login_handler,

    ),
    components(
        schemas(
            mproduct::schema::AddProductSchema,
            mproduct::schema::UpdateProductSchema,
            mproduct::schema::DeleteProductSchema,
            mproduct::models::ProductModel,
            mauth::schemas::LoginUserSchema,
            musers::models::MUserModel,
            musers::models::UserRole,
            
        )
    ),
    tags(
        (name = "Products", description = "APIs for managing products"),
        (name = "Authentication", description = "APIs for user authentication"),
        (name = "Users", description = "APIs for managing users"),
    )
)]
pub struct ApiDoc;
pub fn create_router(app_state: AppState) -> Router {
    return Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest(
                    "/auth",
                    mauth::routes::create_auth_router(app_state.clone()),
                )
                .nest(
                    "/users",
                    musers::routes::create_user_router(app_state.clone()),
                )
                .nest(
                    "/products",
                    mproduct::routes::create_prod_router(app_state.clone()),
                )
                .merge(
                    SwaggerUi::new("/swagger")
                        .url("/api-docs/openapi.json", ApiDoc::openapi().clone()),
                ),
        )
        .merge(
            SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi().clone()),
        );
}
