use axum::{body::Body, http::Request, response::Response};

use crate::{
    AppState,
    mauth::middlewares::{auth_middleware, auth_middleware_with_admin_perms},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MyAuthPermsLayer {}

impl<S> Layer<S> for MyAuthPermsLayer {
    type Service = MyAuthPermsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyAuthPermsService { inner }
    }
}

#[derive(Clone)]
pub struct MyAuthPermsService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MyAuthPermsService<S>
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
        let mut inner = self.inner.clone();
        Box::pin(async move {
            match auth_middleware_with_admin_perms(req).await {
                Ok(req2) => inner.call(req2).await,
                Err(resp) => Ok(resp),
            }
        })
    }
}

#[derive(Clone)]
pub struct MyAuthLayer {
    pub state: AppState,
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
pub struct MyAuthService<S> {
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
