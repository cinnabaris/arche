use std::net::SocketAddr;

use juniper;

use super::super::orm::PooledConnection as Db;

pub struct Context {
    pub locale: String,
    pub remote: SocketAddr,
    pub db: Db,
}

impl juniper::Context for Context {}
