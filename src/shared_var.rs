use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use axum::{Json, Router};
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::{AppState, mauth, mproduct, musers};
use crate::util::helpers::map_pg_database_error;


// 
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
        pub fn db_err(e: sqlx::Error) -> Self {
        use sqlx::error::DatabaseError;
        match e {
            sqlx::Error::RowNotFound => Self {
                code: 404,
                message: "Record not found".into(),
                data: None,
            },
            sqlx::Error::Database(db) => {
                let error = map_pg_database_error(db.as_ref());
                
                Self {
                    code: error.code.parse().unwrap_or(500),
                    message: error.message,
                    data: None,
                }
            }
            _ => Self {
                code: 500,
                message: format!("Database error: {}", e),
                data: None,
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

#[derive(Debug, Default, Clone, serde::Deserialize, ToSchema, IntoParams)]
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
