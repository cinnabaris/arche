#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::{schema, Connection, Dao, Pool, DRIVER};

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::{schema, Connection, Dao, Pool, DRIVER};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[cfg(feature = "postgresql")]
    pub postgresql: postgresql::Config,
    #[cfg(feature = "mysql")]
    pub mysql: mysql::Config,
}
