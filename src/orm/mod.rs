#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::{schema, Config, Connection, DRIVER};

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::{schema, Config, Connection, DRIVER};

use std::ops::Deref;

use diesel::r2d2::ConnectionManager;
use r2d2;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub struct PooledConnection(pub r2d2::PooledConnection<ConnectionManager<Connection>>);

impl<'a, 'r> FromRequest<'a, 'r> for PooledConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(c) => Outcome::Success(PooledConnection(c)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
