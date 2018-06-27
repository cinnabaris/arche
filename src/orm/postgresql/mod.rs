pub mod schema;

use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

use super::super::errors::Result;

pub const DRIVER: &'static str = "postgresql";

// logging:
// edit "/var/lib/postgres/data/postgresql.conf", change "log_statement = 'all'"
// sudo gpasswd -a YOUR-NAME wheel
// journalctl -f -u postgresql
// show database size: /l+
// \c arche
// \d xxx
pub type Connection = PgConnection;

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
    logging:
    edit "/var/lib/postgres/data/postgresql.conf", change "log_statement = 'all'"
    sudo gpasswd -a YOUR-NAME wheel
    journalctl -f -u postgresql
    show database size: /l+
    \c arche
    \d xxx
    */
    pub fn open(&self) -> Result<Pool<ConnectionManager<PgConnection>>> {
        let url = format!(
            "postgres://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            name = self.name,
            host = self.host,
            port = self.port,
        );
        Ok(Pool::new(ConnectionManager::<PgConnection>::new(&url[..]))?)
    }
}
