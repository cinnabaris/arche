use super::cache::Cache;
use super::jwt::Jwt;
use super::queue::Queue;
use super::repositories::Repository;
use super::security::Security;

pub struct Context {
    pub repository: Box<Repository>,
    pub cache: Box<Cache>,
    pub queue: Box<Queue>,
    pub security: Box<Security>,
    pub jwt: Jwt,
}

impl Context {
    // pub fn new(r: Box<Repository>, c: Box<Cache>, q: Box<Queue>) -> Context {
    //     Context {
    //         repository: r,
    //         cache: c,
    //         queue: q,
    //     }
    // }
}
