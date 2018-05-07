use base64;
use sha2::{self, Digest};
use sodiumoxide::{self, crypto::pwhash, crypto::secretbox, randombytes};

use super::result::{Error, Result};

pub trait Security: Send + Sync + Encryptor + Password + Ssha512 {}

pub trait Encryptor: Send + Sync {
    fn random_bytes(&self, l: usize) -> Vec<u8>;
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}

pub trait Password: Send + Sync {
    fn sum(&self, plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, cipher: &[u8], plain: &[u8]) -> bool;
}

// https://wiki.dovecot.org/Authentication/PasswordSchemes
// https://www.tunnelsup.com/using-salted-sha-hashes-with-dovecot-authentication/
// doveadm pw -t {SSHA256.hex}4a847fefc4f9ab450f16783c5025d64313942a1ceb2599707cdb65940ba901e513fa442f -p pass
pub trait Ssha512: Send + Sync {
    fn sum(&self, plain: &[u8], len: usize) -> String;
    fn verify(&self, cipher: String, plain: &[u8]) -> bool;
}

//-----------------------------------------------------------------------------

// #[derive(Clone)]
pub struct Sodium {
    key: secretbox::Key,
}

impl Security for Sodium {}

impl Sodium {
    pub fn new(key: &[u8]) -> Result<Self> {
        sodiumoxide::init();
        match secretbox::Key::from_slice(key) {
            Some(key) => Ok(Self { key: key }),
            None => Err(Error::WithDescription(s!("bad key length, must be 32"))),
        }
    }
}

impl Encryptor for Sodium {
    fn random_bytes(&self, l: usize) -> Vec<u8> {
        randombytes::randombytes(l)
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
                Err(_) => Err(Error::WithDescription(s!("decrypt data"))),
            },
            None => Err(Error::WithDescription(s!("none nonce"))),
        }
    }
}

impl Password for Sodium {
    fn sum(&self, plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err(Error::WithDescription(s!("sum hash"))),
        }
    }

    fn verify(&self, cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
}

impl Sodium {
    // https://github.com/RustCrypto/hashes
    fn _sum_ssha512(plain: &[u8], salt: &[u8]) -> String {
        // only sha2
        let mut hasher = sha2::Sha512::default();
        hasher.input(plain);
        hasher.input(salt);
        let mut out = hasher.result().as_slice().to_vec();
        out.extend(salt);
        return base64::encode(&out);
    }
}

impl Ssha512 for Sodium {
    fn sum(&self, plain: &[u8], len: usize) -> String {
        let salt = self.random_bytes(len);
        return Self::_sum_ssha512(plain, &salt);
    }

    fn verify(&self, cipher: String, plain: &[u8]) -> bool {
        match base64::decode(&cipher) {
            Ok(buf) => cipher == Self::_sum_ssha512(plain, &buf[64..]),
            Err(_) => false,
        }
    }
}
