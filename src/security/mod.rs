#[cfg(feature = "sodium")]
pub mod sodium;

use super::result::Result;

pub trait HashBox {
    fn random_bytes(&self, l: usize) -> Vec<u8>;
    fn sum(&self, plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, cipher: &[u8], plain: &[u8]) -> bool;
}

pub trait SecretBox {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}
