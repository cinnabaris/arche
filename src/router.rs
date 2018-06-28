use std::error::Error as StdError;
use std::sync::Arc;

use futures::future;
use hyper::{
    self, header::{HeaderValue, CONTENT_TYPE}, rt::Future, service::{NewService, Service}, Body,
    Method, Request, Response, StatusCode,
};
use mime;
use regex::Regex;

use super::{context::Context, errors::Result, graphql};

pub fn routes() -> Result<Vec<(Method, Regex, Box<Route>)>> {
    let mut items: Vec<(Method, Regex, Box<Route>)> = Vec::new();
    items.push((
        Method::GET,
        Regex::new(r"^/doc$")?,
        Box::new(graphql::routes::Doc {}),
    ));
    Ok(items)
}

pub trait Route: Sync + Send {
    fn handle(
        &self,
        ctx: Arc<Context>,
        req: &Request<Body>,
    ) -> Result<(mime::Mime, Response<Body>)>;
}

pub struct Router {
    ctx: Arc<Context>,
    routes: Arc<Vec<(Method, Regex, Box<Route>)>>,
}

impl Router {
    fn new(ctx: Arc<Context>, routes: Arc<Vec<(Method, Regex, Box<Route>)>>) -> Self {
        Self {
            ctx: ctx,
            routes: routes,
        }
    }
    fn handle(&self, req: &Request<Body>) -> Result<(mime::Mime, Response<Body>)> {
        let method = req.method();
        let uri = req.uri();
        info!("{:?} {} {}", req.version(), method, uri);
        let path = uri.path();
        for (m, p, h) in self.routes.iter() {
            if m == method && p.is_match(path) {
                info!("match {}", p);
                return h.handle(Arc::clone(&self.ctx), req);
            }
        }
        let mut res = Response::new(Body::empty());
        *res.status_mut() = StatusCode::NOT_FOUND;
        Ok((mime::TEXT_PLAIN_UTF_8, res))
    }
}

impl Service for Router {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (mty, mut res) = match self.handle(&req) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                let mut res = Response::new(Body::empty());
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                (mime::TEXT_PLAIN_UTF_8, res)
            }
        };
        if let Ok(t) = HeaderValue::from_str(format!("{}", mty).as_str()) {
            res.headers_mut().insert(CONTENT_TYPE, t);
        }
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
    routes: Arc<Vec<(Method, Regex, Box<Route>)>>,
}

impl RouterBuilder {
    pub fn new(ctx: Arc<Context>, routes: Arc<Vec<(Method, Regex, Box<Route>)>>) -> Self {
        Self {
            ctx: ctx,
            routes: routes,
        }
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
        Box::new(future::ok(Router::new(
            Arc::clone(&self.ctx),
            Arc::clone(&self.routes),
        )))
    }
}
