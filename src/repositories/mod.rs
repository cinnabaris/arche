pub mod mysql;
pub mod postgresql;

use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use r2d2::Pool;

use super::i18n;
use super::result::Result;

pub trait Repository: Send + Sync + i18n::Repository {}

pub struct PostgreSql {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgreSql {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool: pool }
    }
    pub fn db(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.pool.get()?)
    }
}

impl Repository for PostgreSql {}

//-----------------------------------------------------------------------------

pub struct MySql {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl MySql {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Self { pool: pool }
    }
    pub fn db(&self) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
        Ok(self.pool.get()?)
    }
}
