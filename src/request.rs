use hyper::header::{Authorization, Bearer, Header, Host, Raw};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request,
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
        return Outcome::Failure((Status::BadRequest, ()));
    }
}

//-----------------------------------------------------------------------------
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
