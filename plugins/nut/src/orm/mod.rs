pub mod postgresql;

use chrono::NaiveDateTime;

use super::errors::Result;

pub trait Migration {
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<(String, NaiveDateTime)>>;
    fn dump(&self) -> Result<()>;
    fn restore(&self) -> Result<()>;
}
