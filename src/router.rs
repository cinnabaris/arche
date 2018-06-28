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
    ) -> Result<(StatusCode, mime::Mime, Option<Body>)>;
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
    fn handle(&self, req: &Request<Body>) -> Result<(StatusCode, mime::Mime, Option<Body>)> {
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
        Ok((StatusCode::NOT_FOUND, mime::TEXT_PLAIN_UTF_8, None))
    }
}

impl Service for Router {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (code, content_type, body) = match self.handle(&req) {
            Ok(v) => v,
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                mime::TEXT_PLAIN_UTF_8,
                Some(Body::from(format!("{:?}", e))),
            ),
        };
        info!("{} {}", code, content_type);
        let mut res = Response::new(Body::empty());
        if let Ok(t) = HeaderValue::from_str(content_type.type_().as_str()) {
            res.headers_mut().insert(CONTENT_TYPE, t);
        }
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
