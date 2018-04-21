extern crate arche;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

use hyper::{Method, StatusCode};
use serde_json::Value;

mod common;

#[test]
fn users() {
    common::html(&"/".to_string(), StatusCode::Ok);
    common::api(
        &"/users/sign-up".to_string(),
        Method::Post,
        None,
        None::<Value>,
        StatusCode::BadRequest,
    );
}
