use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use frank_jwt::{decode, encode, Algorithm};
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

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
#[derive(Clone)]
pub struct Jwt {
    key: String,
    alg: Algorithm,
}

impl Jwt {
    pub fn new(key: String, alg: Algorithm) -> Jwt {
        return Jwt { key: key, alg: alg };
    }

    pub fn sum(&self, payload: &mut Value, ttl: Duration) -> Result<String> {
        let nbf = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let exp = nbf.add(ttl);
        payload["nbf"] = json!(nbf.as_secs());
        payload["exp"] = json!(exp.as_secs());
        let token = encode(json!({}), &self.key, payload, self.alg)?;
        return Ok(token);
    }

    pub fn parse(&self, token: &String) -> Result<Value> {
        let (header, payload) = decode(token, &self.key, self.alg)?;
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
