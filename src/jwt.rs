use std::ops::Add;
use std::result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use frank_jwt::{decode, encode, error, Algorithm, ToKey};
use hyper::header::{Header, Host};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use serde_json::Value;

use super::result::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Home(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Home {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let scheme = req.headers().get_one("X-Forwarded-Proto").unwrap_or("http");
        if let Some(host) = req.headers().get_one(Host::header_name()) {
            return Outcome::Success(Home(format!("{}://{}", scheme, host)));
        }
        return Outcome::Failure((Status::BadRequest, ()));
    }
}

//-----------------------------------------------------------------------------

struct Key(Vec<u8>);

impl ToKey for Key {
    fn to_key(&self) -> result::Result<Vec<u8>, error::Error> {
        let Key(k) = self;
        Ok(k.to_vec())
    }
}

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub struct Jwt {
    key: Key,
    alg: Algorithm,
}

impl Jwt {
    pub fn new(key: &[u8], alg: Algorithm) -> Jwt {
        return Jwt {
            key: Key(key.to_vec()),
            alg: alg,
        };
    }

    pub fn sum(&self, payload: &mut Value, ttl: Duration) -> Result<String> {
        let nbf = try!(SystemTime::now().duration_since(UNIX_EPOCH));
        let exp = nbf.add(ttl);
        payload["nbf"] = json!(nbf.as_secs());
        payload["exp"] = json!(exp.as_secs());
        let token = try!(encode(json!({}), &self.key, payload, self.alg));
        return Ok(token);
    }

    pub fn parse(&self, token: &String) -> Result<Value> {
        let (header, payload) = try!(decode(token, &self.key, self.alg));
        if let Some(nbf) = payload["nbf"].as_u64() {
            if let Some(exp) = payload["exp"].as_u64() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                if now >= nbf && now <= exp {
                    return Ok(payload);
                }
            }
        }
        return Err(Error::WithDescription(format!("jwt: {:?}", header)));
    }
}
