use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use axum::{Json, Router};
use utoipa::openapi::SecurityRequirement;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa::openapi::{Components, security::SecurityScheme, security::HttpAuthScheme, security::Http};
use utoipa_swagger_ui::SwaggerUi;

use utoipa::Modify;

    

use crate::{AppState, mauth, mcart, mproduct, musers};
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

#[derive(Debug, Default, Clone, serde::Deserialize, ToSchema, IntoParams, PartialEq)]
pub struct FilterOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(

        mauth::handlers::user_login_handler,
        musers::handlers::get_users_handler,
        musers::handlers::create_new_user_handler,
        musers::handlers::update_users_handler,
        musers::handlers::delete_users_handler,
        mproduct::handlers::get_product_handler,
        mproduct::handlers::add_product_handler,
        mproduct::handlers::update_product_handler,
        mproduct::handlers::del_product_handler,
        mcart::handlers::create_cart_handler,
        mcart::handlers::get_cart_by_user_handler,
        mcart::handlers::get_open_cart_by_user_handler,
        mcart::handlers::add_item_to_cart_handler,
        mcart::handlers::update_item_in_cart_handler,
        mcart::handlers::delete_items_from_cart_handler,
        mcart::handlers::verify_cart_handler,
        mcart::handlers::checkout_cart_handler,



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
            musers::schema::AddUserSchema,
            musers::schema::UpdateUsersSchema,
            musers::schema::DeleteUsersSchema,
            FilterOptions,
            MyBaseResponse::<mproduct::models::ProductModel>,
            MyBaseResponse::<musers::models::MUserModel>,
            MyBaseResponse<Vec<mproduct::models::ProductModel>>,
            MyBaseResponse<Vec<musers::models::MUserModel>>,
            mcart::schemas::AddCartItemSchema,
            mcart::schemas::UpdateCartItemSchema,
            mcart::schemas::DeleteCartItemSchema,
            mcart::schemas::CreateCartSchema,
            mcart::schemas::UpdateCartStatusSchema,
            mcart::schemas::ClearCartSchema,
            mcart::schemas::CheckoutCartSchema,
            mcart::schemas::GetCartByUserSchema,
            MyBaseResponse::<mcart::models::CartModel>,
            MyBaseResponse::<mcart::models::CartItemModel>,
            MyBaseResponse::<mcart::models::CartWithItemsModel>,
            MyBaseResponse::<Vec<mcart::models::CartItemWithProductModel>>,
            mcart::models::CartModel,
            mcart::models::CartItemModel,
            mcart::models::CartWithItemsModel,
            mcart::models::CartItemWithProductModel,
            
            
        )
    ),
    tags(
        (name = "Products", description = "APIs for managing products"),
        (name = "Authentication", description = "APIs for user authentication"),
        (name = "Users", description = "APIs for managing users"),
        (name = "Carts", description = "APIs for managing shopping carts")
    ),
    modifiers(&SecurityAddon),

)]

pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth", 
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
       openapi.security = Some(vec![ SecurityRequirement::new("bearerAuth", Vec::<String>::new())]);
    }
}

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
                .nest("/cart", mcart::routes::create_cart_router(app_state.clone()),)
                .merge(
                    SwaggerUi::new("/swagger")
                        .url("/api-docs/openapi.json", ApiDoc::openapi().clone()),
                ),
        )
        .merge(
            SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}
