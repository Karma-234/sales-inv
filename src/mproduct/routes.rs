use axum::{
    Router,
    body::Body,
    extract::{Query, State},
    http::Request,
    response::Response,
    routing::{delete, get, post, put},
};

use crate::{
    AppState,
    mauth::middlewares::auth_middleware,
    mproduct::{
        self,
        schema::{AddProductSchema, DeleteProductSchema, UpdateProductSchema},
    },
    shared_var::FilterOptions,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
struct MyAuthLayer {
    state: AppState,
}

impl<S> Layer<S> for MyAuthLayer {
    type Service = MyAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyAuthService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
struct MyAuthService<S> {
    inner: S,
    state: AppState,
}

impl<S> Service<Request<Body>> for MyAuthService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let state = self.state.clone();
        let mut inner = self.inner.clone();
        Box::pin(async move {
            match auth_middleware(req, state).await {
                Ok(req2) => inner.call(req2).await,
                Err(resp) => Ok(resp),
            }
        })
    }
}

pub fn create_prod_router(app: AppState) -> Router {
    return Router::new()
        .route(
            "/get",
            get(
                |pool: axum::extract::State<AppState>,
                 filter: axum::extract::Query<FilterOptions>| async move {
                    let op = Query(FilterOptions {
                        limit: Some(2),
                        page: Some(3),
                        // search: Some("Amoxil".to_string()),
                        search: filter.search.clone(),
                    });
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::get_product_handler(op, State(state)).await;
                },
            ),
        )
        .route(
            "/update",
            put(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<UpdateProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::update_prod_handler(State(state), payload).await;
                },
            ),
        )
        .route(
            "/delete",
            delete(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<DeleteProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::del_prod_handler(payload, State(state)).await;
                },
            ),
        )
        .route(
            "/add",
            post(
                |pool: axum::extract::State<AppState>,
                 payload: axum::extract::Json<AddProductSchema>| async move {
                    let state = AppState {
                        db: pool.0.db,
                        env: pool.0.env,
                    };
                    return mproduct::handlers::add_product_handler(payload, State(state)).await;
                },
            ),
        )
        .layer(MyAuthLayer { state: app.clone() })
        .with_state(app);
}
