use std::net::SocketAddr;

use juniper;

pub struct Context {
    pub locale: String,
    pub remote: SocketAddr,
}

impl juniper::Context for Context {}
