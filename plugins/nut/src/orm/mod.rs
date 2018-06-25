pub mod mysql;
pub mod postgresql;

use chrono::NaiveDateTime;
use diesel::{mysql::MysqlConnection, pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool as R2d2Pool, PooledConnection as R2d2PooledConnection};

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgresql: Option<postgresql::Config>,
    pub mysql: Option<mysql::Config>,
}

pub enum Pool {
    PostgreSql(R2d2Pool<ConnectionManager<PgConnection>>),
    MySql(R2d2Pool<ConnectionManager<MysqlConnection>>),
}

impl Pool {
    pub fn new(cfg: &Config) -> Result<Self> {
        if let Some(ref cfg) = cfg.postgresql {
            return Ok(Pool::PostgreSql(cfg.open()?));
        }
        if let Some(ref cfg) = cfg.mysql {
            return Ok(Pool::MySql(cfg.open()?));
        }
        Err("bad database provider".into())
    }

    pub fn get(&self) -> Result<Connection> {
        match self {
            Pool::PostgreSql(p) => Ok(Connection::PostgreSql(p.get()?)),
            Pool::MySql(p) => Ok(Connection::MySql(p.get()?)),
        }
    }
}

pub enum Connection {
    PostgreSql(R2d2PooledConnection<ConnectionManager<PgConnection>>),
    MySql(R2d2PooledConnection<ConnectionManager<MysqlConnection>>),
}

pub trait Migration {
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<(String, NaiveDateTime)>>;
    fn dump(&self) -> Result<()>;
    fn restore(&self) -> Result<()>;
}
