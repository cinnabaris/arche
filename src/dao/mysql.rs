use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager};
use r2d2::Pool;

use super::super::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn url(&self) -> String {
        format!(
            "mysql://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            name = self.name,
            host = self.host,
            port = self.port,
        )
    }
}

pub struct Dao {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Dao {
    pub fn new(url: &String) -> Result<Self> {
        Ok(Self {
            pool: Pool::new(ConnectionManager::<MysqlConnection>::new(&url[..]))?,
        })
    }
}
