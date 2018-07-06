use std::ops::Add;

use chrono::{Duration, Utc};
use frank_jwt::{decode, encode, Algorithm};
use serde_json::Value;

use super::errors::Result;

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub struct Jwt {
    key: String,
    alg: Algorithm,
}

impl Jwt {
    pub fn new(key: String, alg: Algorithm) -> Jwt {
        return Jwt { key: key, alg: alg };
    }

    pub fn sum(&self, payload: &mut Value, ttl: Duration) -> Result<String> {
        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(ttl);
        payload["nbf"] = json!(nbf.timestamp());
        payload["exp"] = json!(exp.timestamp());
        match encode(json!({}), &self.key, payload, self.alg) {
            Ok(t) => Ok(t),
            Err(_) => Err("generate jwt failed".into()),
        }
    }

    pub fn parse(&self, token: &String) -> Result<Value> {
        if let Ok((_header, payload)) = decode(token, &self.key, self.alg) {
            if let Some(nbf) = payload["nbf"].as_i64() {
                if let Some(exp) = payload["exp"].as_i64() {
                    let now = Utc::now().naive_utc().timestamp();
                    if now >= nbf && now <= exp {
                        return Ok(payload);
                    }
                }
            }
        }
        return Err("bad jwt token".into());
    }
}
