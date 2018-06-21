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
