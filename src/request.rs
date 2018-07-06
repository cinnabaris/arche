use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use diesel::prelude::*;
use hyper::header::{AcceptLanguage, Authorization, Bearer, Header, Host, LanguageTag, Raw};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request,
};
use url::Url;

use super::{
    errors::Result,
    orm::{schema::locales, PooledConnection as Db},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Home(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Home {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let scheme = req.headers().get_one("X-Forwarded-Proto").unwrap_or("http");
        if let Some(host) = req.headers().get_one(Host::header_name()) {
            return Outcome::Success(Home(format!("{}://{}", scheme, host)));
        }
        Outcome::Failure((Status::BadRequest, ()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if let Some(auth) = req
            .headers()
            .get_one(Authorization::<Bearer>::header_name())
        {
            if let Ok(auth) = Authorization::<Bearer>::parse_header(&Raw::from(auth)) {
                let Authorization::<Bearer>(bearer) = auth;
                return Outcome::Success(Token(Some(bearer.token)));
            }
        }
        Outcome::Success(Token(None))
    }
}

// https://tools.ietf.org/html/bcp47
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locale(pub String);

impl<'a, 'r> Locale {
    fn parse(req: &'a Request<'r>) -> Result<Option<LanguageTag>> {
        let key = String::from("locale");
        // 1. Check URL arguments.
        let url = Url::parse(&format!("{}{}", "fake:/", req.uri().as_str()))?;
        let pairs: HashMap<String, String> = url.query_pairs().into_owned().collect();
        if let Some(l) = pairs.get(&key) {
            return Ok(Some(LanguageTag::from_str(l)?));
        }
        // 2. Get language information from cookies.
        if let Some(ck) = req.cookies().get(&key[..]) {
            return Ok(Some(LanguageTag::from_str(ck.value())?));
        }
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(al) = req.headers().get_one(AcceptLanguage::header_name()) {
            if let Ok(al) = AcceptLanguage::parse_header(&Raw::from(al)) {
                if let Some(al) = al.first() {
                    return Ok(Some(al.item.clone()));
                }
            }
        }

        Ok(None)
    }

    fn match_(req: &'a Request<'r>) -> Option<String> {
        if let Ok(lng) = Self::parse(req) {
            if let Some(lng) = lng {
                if let Outcome::Success(db) = req.guard::<Db>() {
                    let lng = format!("{}", lng);
                    if let Ok(c) = locales::dsl::locales
                        .filter(locales::dsl::lang.eq(&lng))
                        .count()
                        .get_result::<i64>(db.deref())
                    {
                        if c > 0 {
                            return Some(lng);
                        }
                    }
                }
            }
        }
        None
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let lng = match Self::match_(req) {
            Some(v) => v,
            None => "en-US".to_string(),
        };
        Outcome::Success(Locale(lng))
    }
}
