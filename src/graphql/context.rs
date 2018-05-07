use std::net::SocketAddr;

use juniper;

pub struct Context {
    pub locale: String,
    pub remote: SocketAddr,
}

impl Context {
    pub fn new(locale: String, remote: SocketAddr) -> Self {
        Self {
            locale: locale,
            remote: remote,
        }
    }
}

impl juniper::Context for Context {}
