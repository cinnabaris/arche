use sodiumoxide::crypto::secretbox;

use super::errors::Result;

// https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html
pub const DATE_FORMAT: &'static str = "%Y-%m-%d";
pub const TIME_FORMAT: &'static str = "%H:%M:%S";
// 2001-07-08T00:34:60.026490+09:30 	ISO 8601 / RFC 3339 date & time format.
pub const DATETIME_FORMAT: &'static str = "%+";

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
    pub fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(&plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    pub fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(v) => Ok(v),
                Err(_) => Err("decrypt data failed".into()),
            },
            None => Err("bad nonce".into()),
        }
    }
}

// ----------------------------------------------------------------------------

pub mod hash {

    use sodiumoxide::crypto::pwhash;

    use super::super::errors::Result;

    pub fn sum(plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err("build password failed".into()),
        }
    }

    pub fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
}

// ----------------------------------------------------------------------------

pub mod random {
    use sodiumoxide::randombytes;

    pub fn bytes(l: usize) -> Vec<u8> {
        randombytes::randombytes(l)
    }
}

// ----------------------------------------------------------------------------

pub mod ssha512 {
    // https://wiki.dovecot.org/Authentication/PasswordSchemes
    // https://www.tunnelsup.com/using-salted-sha-hashes-with-dovecot-authentication/
    // doveadm pw -t {SSHA256.hex}4a847fefc4f9ab450f16783c5025d64313942a1ceb2599707cdb65940ba901e513fa442f -p pass

    use base64;
    use sha2::{self, Digest};

    // https://github.com/RustCrypto/hashes
    pub fn sum(plain: &[u8], salt: &[u8]) -> String {
        // only sha2
        let mut hasher = sha2::Sha512::default();
        hasher.input(plain);
        hasher.input(salt);
        let mut out = hasher.result().as_slice().to_vec();
        out.extend(salt);
        base64::encode(&out)
    }

    pub fn verify(cipher: String, plain: &[u8]) -> bool {
        match base64::decode(&cipher) {
            Ok(buf) => cipher == sum(plain, &buf[64..]),
            Err(_) => false,
        }
    }
}
