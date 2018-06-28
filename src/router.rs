use std::error::Error as StdError;
use std::sync::Arc;

use futures::future;
use hyper::{
    self, rt::Future, service::{NewService, Service}, Body, Method, Request, Response, StatusCode,
};

use super::{context::Context, errors::Result};

type BoxFut = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

pub struct Router {
    ctx: Arc<Context>,
}

impl Router {
    fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }
    fn handle(&self, req: Request<Body>) -> Result<(Option<Body>, StatusCode)> {
        let method = req.method();
        let uri = req.uri();
        info!("{:?} {} {}", req.version(), method, uri);
        match (method, uri.path()) {
            (&Method::GET, "/doc") => {
                // TODO graphql doc
                Ok((
                    Some(Body::from("Try POSTing data to /echo")),
                    StatusCode::OK,
                ))
            }
            (&Method::POST, "/graphql") => {
                // TODO graphql handle
                Ok((
                    Some(Body::from("Try POSTing data to /echo")),
                    StatusCode::OK,
                ))
            }
            _ => Ok((None, StatusCode::NOT_FOUND)),
        }
    }
}

impl Service for Router {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = BoxFut;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (body, code) = match self.handle(req) {
            Ok(v) => v,
            Err(e) => (
                Some(Body::from(format!("{:?}", e))),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        let mut res = Response::new(Body::empty());
        if let Some(b) = body {
            *res.body_mut() = b;
        }
        *res.status_mut() = code;
        Box::new(future::ok(res))
    }
}

impl future::IntoFuture for Router {
    type Future = future::FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = hyper::Error;

    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

pub struct RouterBuilder {
    ctx: Arc<Context>,
}

impl RouterBuilder {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx: ctx }
    }
}

impl NewService for RouterBuilder {
    type Error = hyper::Error;
    type ReqBody = Body;
    type ResBody = Body;
    type Service = Router;
    type InitError = Box<StdError + Send + Sync>;
    type Future = Box<Future<Item = Self::Service, Error = Self::InitError> + Send>;

    fn new_service(&self) -> Self::Future {
        let ctx = Arc::clone(&self.ctx);
        Box::new(future::ok(Router::new(ctx)))
    }
}

pub fn routes() -> Vec<(String, String, String)> {
    let items = Vec::new();

    items
}
