use std::error::Error as StdError;
use std::sync::Arc;

use futures::{future, Stream};
use http::request::Parts;
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
    items.push((
        Method::POST,
        Regex::new(r"^/graphql$")?,
        Box::new(graphql::routes::GraphQL {}),
    ));
    Ok(items)
}

pub trait Route: Sync + Send {
    fn handle(
        &self,
        ctx: Arc<Context>,
        parts: &Parts,
        body: &Vec<u8>,
    ) -> Result<(StatusCode, mime::Mime, Option<String>)>;
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
}

fn handle(
    ctx: Arc<Context>,
    parts: &Parts,
    body: &Vec<u8>,
    routes: Arc<Vec<(Method, Regex, Box<Route>)>>,
) -> Result<(StatusCode, mime::Mime, Option<String>)> {
    let pth = parts.uri.path();
    for (m, p, h) in routes.iter() {
        if m == &parts.method && p.is_match(pth) {
            info!("match {}", p);
            return h.handle(ctx, parts, body);
        }
    }
    Ok((StatusCode::NOT_FOUND, mime::TEXT_PLAIN_UTF_8, None))
}

impl Service for Router {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let ctx = Arc::clone(&self.ctx);
        let routes = Arc::clone(&self.routes);
        let (parts, body) = req.into_parts();
        info!("{:?} {} {}", parts.version, parts.method, parts.uri);
        let reversed = body.concat2().map(move |chunk| {
            let body = chunk.iter().cloned().collect::<Vec<u8>>();
            let (sc, ct, bd) = match handle(ctx, &parts, &body, routes) {
                Ok(v) => v,
                Err(e) => {
                    error!("{:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        mime::TEXT_PLAIN_UTF_8,
                        None,
                    )
                }
            };
            info!("{} {}", sc, ct);
            let mut res = Response::new(Body::empty());
            *res.status_mut() = sc;
            if let Ok(t) = HeaderValue::from_str(&format!("{}", ct)[..]) {
                res.headers_mut().insert(CONTENT_TYPE, t);
            }
            if let Some(b) = bd {
                *res.body_mut() = Body::from(b);
            }
            res
        });
        Box::new(reversed)
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
