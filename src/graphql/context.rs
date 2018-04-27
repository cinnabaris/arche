use juniper;

use super::super::orm::Pool;

pub struct Context {
    pool: Pool,
}

impl Context {
    pub fn new(pool: Pool) -> Self {
        Self { pool: pool }
    }
}

impl juniper::Context for Context {}
