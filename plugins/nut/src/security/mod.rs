pub mod sodium;

use super::errors::Result;

pub trait Encryptor {
    fn random_bytes(l: usize) -> Vec<u8>;
    fn password(plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(cipher: &[u8], plain: &[u8]) -> bool;
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}
