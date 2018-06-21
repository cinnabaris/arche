pub mod migration;
pub mod schema;

use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

use super::super::result::Result;

#[cfg(feature = "mysql")]
pub const DRIVER: &'static str = "mysql";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn open(&self) -> Result<Pool<ConnectionManager<MysqlConnection>>> {
        let url = format!(
            "mysql://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            name = self.name,
            host = self.host,
            port = self.port,
        );
        Ok(Pool::new(ConnectionManager::<MysqlConnection>::new(
            &url[..],
        ))?)
    }
}

pub struct Dao {
    pub db: PooledConnection<ConnectionManager<MysqlConnection>>,
}

impl Dao {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Result<Self> {
        Ok(Self { db: pool.get()? })
    }
}
