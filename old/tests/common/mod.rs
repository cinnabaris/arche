use std::str;

use futures::future::Future;
use hyper::header::{Authorization, Bearer, ContentLength, ContentType};
use hyper::{Body, Chunk, Client, Method, Request, StatusCode};
use serde::ser::Serialize;
use serde_json::{self, Value};
use tokio_core::reactor::Core;

pub fn html(uri: &String, status: StatusCode) {
    let body = http(uri, Method::Get, None, None::<Value>, status);
    println!("{}", str::from_utf8(&body).unwrap());
}

pub fn api<B: Serialize>(
    uri: &String,
    method: Method,
    token: Option<String>,
    body: Option<B>,
    status: StatusCode,
) -> Option<Value> {
    let body = http(&format!("/api{}", uri), method, token, body, status);
    match status {
        StatusCode::Ok => Some(serde_json::from_slice(&body).unwrap()),
        _ => {
            println!("{}", str::from_utf8(&body).unwrap());
            None
        }
    }
}

fn http<B: Serialize>(
    uri: &String,
    method: Method,
    token: Option<String>,
    body: Option<B>,
    status: StatusCode,
) -> Chunk {
    let url = format!("http://localhost:8080{}", uri);
    println!("HTTP {:?} {:?}", method, url);
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = url.parse().unwrap();
    let mut req: Request<Body> = Request::new(method, uri);
    req.headers_mut().set(ContentType::json());
    if let Some(token) = token {
        req.headers_mut()
            .set(Authorization(Bearer { token: token }));
    }

    if let Some(body) = body {
        let body = serde_json::to_vec(&body).unwrap();
        req.headers_mut().set(ContentLength(body.len() as u64));
        req.set_body(body);
    }

    let call = client.request(req).and_then(|res| {
        println!("{}", res.status());
        assert_eq!(status, res.status());
        res.body().concat2()
    });

    core.run(call).unwrap()
}
