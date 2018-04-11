use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use diesel;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

use super::result::Result;

/*
logging:
edit "/var/lib/postgres/data/postgresql.conf", change "log_statement = 'all'"
sudo gpasswd -a YOUR-NAME wheel
journalctl -f -u postgresql
*/

#[cfg(feature = "postgresql")]
type DiselConnection = diesel::pg::PgConnection;
#[cfg(feature = "mysql")]
type DiselConnection = diesel::mysql::MysqlConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DiselConnection>>;

pub fn pool(url: &String) -> Result<Pool> {
    let p = r2d2::Pool::new(ConnectionManager::<DiselConnection>::new(&url[..]))?;
    Ok(p)
}

pub struct Connection(pub PooledConnection<ConnectionManager<DiselConnection>>);

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
    type Target = DiselConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
