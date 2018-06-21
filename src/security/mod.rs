#[cfg(feature = "sodium")]
pub mod sodium;
#[cfg(feature = "sodium")]
pub use self::sodium::{random_bytes, sum, verify};

use super::result::Result;

pub trait SecretBox {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}
