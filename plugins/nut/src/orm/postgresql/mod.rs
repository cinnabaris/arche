pub mod schema;

use std::ops::Deref;

use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{self, PooledConnection};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::super::errors::Result;

pub const DRIVER: &'static str = "postgresql";

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
    pub fn open(&self) -> Result<Pool> {
        let url = format!(
            "postgres://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            name = self.name,
            host = self.host,
            port = self.port,
        );
        Ok(r2d2::Pool::new(ConnectionManager::<PgConnection>::new(
            &url[..],
        ))?)
    }
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(c) => Outcome::Success(Connection(c)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Dao<'a> {
    pub db: &'a PgConnection,
}

impl<'a> Dao<'a> {
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db: db }
    }
}
