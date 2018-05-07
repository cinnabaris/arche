use sodiumoxide::{crypto::secretbox, randombytes};

use super::result::{Error, Result};

pub fn random_bytes(l: usize) -> Vec<u8> {
    randombytes::randombytes(l)
}

#[derive(Clone)]
pub struct Encryptor {
    key: secretbox::Key,
}

impl Encryptor {
    pub fn new(key: &[u8]) -> Result<Self> {
        match secretbox::Key::from_slice(key) {
            Some(key) => Ok(Self { key: key }),
            None => Err(Error::WithDescription(s!("bad key length, must be 32"))),
        }
    }

    pub fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(&plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    pub fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::WithDescription(s!("decrypt data"))),
            },
            None => Err(Error::WithDescription(s!("none nonce"))),
        }
    }
}

pub mod hash {
    use sodiumoxide::crypto::pwhash;

    use super::super::result::{Error, Result};

    pub fn sum(plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err(Error::WithDescription(s!("sum hash"))),
        }
    }

    pub fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
}

// https://github.com/RustCrypto/hashes
pub mod ssha512 {
    use base64;
    use sha2::{self, Digest};

    pub fn sum(plain: &[u8], len: usize) -> String {
        let salt = super::random_bytes(len);
        return _sum(plain, &salt);
    }

    pub fn verify(cipher: String, plain: &[u8]) -> bool {
        match base64::decode(&cipher) {
            Ok(buf) => cipher == _sum(plain, &buf[64..]),
            Err(_) => false,
        }
    }
    fn _sum(plain: &[u8], salt: &[u8]) -> String {
        // only sha2
        let mut hasher = sha2::Sha512::default();
        hasher.input(plain);
        hasher.input(salt);
        let mut out = hasher.result().as_slice().to_vec();
        out.extend(salt);
        return base64::encode(&out);
    }
}
