use std::ops::Deref;

use diesel;
use r2d2;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use super::result::Result;

#[cfg(feature = "mysql")]
pub type Connection = diesel::mysql::MysqlConnection;
#[cfg(feature = "postgresql")]
pub type Connection = diesel::pg::PgConnection;

#[derive(Clone)]
pub struct Pool(r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>);

impl Pool {
    pub fn new(url: &String) -> Result<Pool> {
        Ok(Pool(r2d2::Pool::new(diesel::r2d2::ConnectionManager::<
            Connection,
        >::new(&url[..]))?))
    }
    pub fn get(&self) -> Result<PooledConnection> {
        let Pool(pool) = self;
        Ok(PooledConnection(pool.get()?))
    }
}

pub struct PooledConnection(r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>);

impl<'a, 'r> FromRequest<'a, 'r> for PooledConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.inner().get() {
            Ok(c) => Outcome::Success(c),
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
