#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::{Connection, Dao, Pool, DRIVER};

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::{Connection, Dao, Pool, DRIVER};

use chrono::NaiveDateTime;

use super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[cfg(feature = "postgresql")]
    pub postgresql: postgresql::Config,
    #[cfg(feature = "mysql")]
    pub mysql: mysql::Config,
}

pub trait Migration {
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<(String, NaiveDateTime)>>;
    fn dump(&self) -> Result<()>;
    fn restore(&self) -> Result<()>;
}
