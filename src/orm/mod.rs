#[cfg(feature = "db-my")]
pub mod mysql;
#[cfg(feature = "db-my")]
pub use self::mysql::{schema, Config, Connection, DRIVER};

#[cfg(feature = "db-pg")]
pub mod postgresql;
#[cfg(feature = "db-pg")]
pub use self::postgresql::{schema, Config, Connection, DRIVER};

use diesel::r2d2::ConnectionManager;
use r2d2;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;
