use chrono::NaiveDateTime;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::*;

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;

use super::result::Result;

pub trait Migration {
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<(String, NaiveDateTime)>>;
    fn dump(&self) -> Result<()>;
    fn restore(&self) -> Result<()>;
}
