use std::net::SocketAddr;
use std::sync::Arc;

use hyper::StatusCode;
use juniper;

use super::super::{context, errors::Result};
use super::Role;

pub struct Context {
    pub locale: String,
    pub token: Option<String>,
    pub remote: SocketAddr,
    pub state: Arc<context::Context>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn current_user(&self) -> Result<i64> {
        if let Some(ref _token) = self.token {
            // TODO
            return Err(format!("{}", StatusCode::UNAUTHORIZED).into());
        }

        Err(format!("{}", StatusCode::NON_AUTHORITATIVE_INFORMATION).into())
    }
    pub fn admin(&self) -> Result<i64> {
        self.must(&Role::Admin.to_string(), &None, &None)
    }
    pub fn must(&self, _name: &String, _type_: &Option<String>, _id: &Option<i64>) -> Result<i64> {
        let _id = self.current_user()?;
        // TODO
        Err(format!("{}", StatusCode::FORBIDDEN).into())
    }
}
