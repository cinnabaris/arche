#[cfg(feature = "sodium")]
use sodiumoxide::{self, crypto::secretbox, randombytes};

use super::result::{Error, Result};

//-----------------------------------------------------------------------------

#[cfg(feature = "sodium")]
pub fn random_bytes(l: usize) -> Vec<u8> {
    randombytes::randombytes(l)
}

//-----------------------------------------------------------------------------

pub mod hash {
    #[cfg(feature = "sodium")]
    use sodiumoxide::crypto::pwhash;

    use super::super::result::{Error, Result};

    #[cfg(feature = "sodium")]
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

    #[cfg(feature = "sodium")]
    pub fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
}

//-----------------------------------------------------------------------------

#[derive(Clone)]
#[cfg(feature = "sodium")]
pub struct Encryptor {
    key: secretbox::Key,
}

#[cfg(feature = "sodium")]
impl Encryptor {
    pub fn new(key: &[u8]) -> Result<Self> {
        sodiumoxide::init();
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

//-----------------------------------------------------------------------------

// https://wiki.dovecot.org/Authentication/PasswordSchemes
// https://www.tunnelsup.com/using-salted-sha-hashes-with-dovecot-authentication/
// doveadm pw -t {SSHA256.hex}4a847fefc4f9ab450f16783c5025d64313942a1ceb2599707cdb65940ba901e513fa442f -p pass
pub mod ssha512 {

    use base64;
    use sha2::{self, Digest};

    use super::random_bytes;

    pub fn sum(plain: &[u8], len: usize) -> String {
        let salt = random_bytes(len);
        return _sum_ssha512(plain, &salt);
    }

    pub fn verify(cipher: String, plain: &[u8]) -> bool {
        match base64::decode(&cipher) {
            Ok(buf) => cipher == _sum_ssha512(plain, &buf[64..]),
            Err(_) => false,
        }
    }

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
