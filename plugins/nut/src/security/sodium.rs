use sodiumoxide::{
    crypto::{pwhash, secretbox},
    randombytes,
};

use super::super::errors::Result;

#[derive(Clone)]
pub struct Encryptor {
    key: secretbox::Key,
}

impl Encryptor {
    pub fn new(key: &[u8]) -> Result<Self> {
        match secretbox::Key::from_slice(key) {
            Some(key) => Ok(Self { key: key }),
            None => Err("bad key length, must be 32".into()),
        }
    }
}

impl super::Encryptor for Encryptor {
    fn random_bytes(l: usize) -> Vec<u8> {
        randombytes::randombytes(l)
    }

    fn password(plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err("build password failed".into()),
        }
    }

    fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(&plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(v) => Ok(v),
                Err(_) => Err("decrypt data failed".into()),
            },
            None => Err("bad nonce".into()),
        }
    }
}
