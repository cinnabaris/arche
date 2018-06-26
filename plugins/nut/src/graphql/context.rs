use std::net::SocketAddr;

use juniper;
use rocket::http::Status;

use super::super::{errors::Result, orm::Connection as Database};
use super::models::Role;

pub struct Context {
    pub locale: String,
    pub token: Option<String>,
    pub remote: SocketAddr,
    pub db: Database,
}

impl juniper::Context for Context {}

impl Context {
    pub fn current_user(&self) -> Result<i64> {
        if let Some(ref _token) = self.token {
            // TODO
            return Err(Status::Unauthorized.reason.into());
        }

        Err(Status::NonAuthoritativeInformation.reason.into())
    }
    pub fn admin(&self) -> Result<i64> {
        self.must(&Role::Admin.to_string(), &None, &None)
    }
    pub fn must(&self, _name: &String, _type_: &Option<String>, _id: &Option<i64>) -> Result<i64> {
        let _id = self.current_user()?;
        // TODO
        Err(Status::Forbidden.reason.into())
    }
}
