pub mod schema;

use diesel::mysql::{r2d2::ConnectionManager, MysqlConnection};
use r2d2::Pool;

use super::super::errors::Result;

pub const DRIVER: &'static str = "mysql";

pub type Connection = MysqlConnection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    /*
    use arche
    desc xxx
    */
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
