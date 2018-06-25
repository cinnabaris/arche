pub mod sodium;

pub use self::sodium::{random_bytes, sum, verify, Encryptor};

use super::errors::Result;

pub trait Provider {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}
