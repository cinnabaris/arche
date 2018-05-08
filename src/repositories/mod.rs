pub mod mysql;
pub mod postgresql;

// use std::ops::Deref;

// use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use r2d2;

use super::result::Result;

pub enum Connection {
    PostgreSql(PooledConnection<ConnectionManager<PgConnection>>),
    // PostgreSql(PgConnection),
    // MySql(MysqlConnection)
}

pub enum Pool {
    PostgreSql(r2d2::Pool<ConnectionManager<PgConnection>>),
    // MySql(r2d2::Pool<ConnectionManager<MysqlConnection>>),
}

impl Pool {
    pub fn get(&self) -> Result<Connection> {
        match self {
            Pool::PostgreSql(pool) => Ok(Connection::PostgreSql(pool.get()?)),
        }
    }
}
