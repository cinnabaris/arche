use sodiumoxide::{
    crypto::{pwhash, secretbox}, randombytes,
};

use super::super::result::{Error, Result};

pub struct HashBox {}

pub fn random_bytes(l: usize) -> Vec<u8> {
    randombytes::randombytes(l)
}

pub fn sum(plain: &[u8]) -> Result<Vec<u8>> {
    match pwhash::pwhash(
        plain,
        pwhash::OPSLIMIT_INTERACTIVE,
        pwhash::MEMLIMIT_INTERACTIVE,
    ) {
        Ok(cip) => Ok(cip[..].to_vec()),
        Err(e) => Err(Error::WithDescription(format!("{:?}", e))),
    }
}

pub fn verify(cipher: &[u8], plain: &[u8]) -> bool {
    match pwhash::HashedPassword::from_slice(cipher) {
        Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
        None => false,
    }
}

#[derive(Clone)]
pub struct SecretBox {
    key: secretbox::Key,
}

impl SecretBox {
    pub fn new(key: &[u8]) -> Result<Self> {
        match secretbox::Key::from_slice(key) {
            Some(key) => Ok(Self { key: key }),
            None => Err(Error::WithDescription(s!("bad key length, must be 32"))),
        }
    }
}

impl super::SecretBox for SecretBox {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(&plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::WithDescription(s!("decrypt data"))),
            },
            None => Err(Error::WithDescription(s!("none nonce"))),
        }
    }
}
