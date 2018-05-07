use std::net::SocketAddr;

use juniper;

use super::super::nut::guards::CurrentUser;
use super::super::orm::Connection;

pub struct Context {
    pub db: Connection,
    pub locale: String,
    pub user: Option<CurrentUser>,
    pub remote: SocketAddr,
}

impl juniper::Context for Context {}
