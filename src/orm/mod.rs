#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::{schema, Config, Connection, DRIVER};

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::{schema, Config, Connection, DRIVER};

use diesel::r2d2::ConnectionManager;
use r2d2;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;
