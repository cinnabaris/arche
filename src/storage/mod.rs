#[cfg(feature = "st-fs")]
pub mod fs;
#[cfg(feature = "st-fs")]
pub use self::fs::{Config, Storage};

#[cfg(feature = "st-aws")]
pub mod s3;
#[cfg(feature = "st-aws")]
pub use self::s3::{Config, Storage};

pub trait Provider {}
