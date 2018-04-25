use hyper::header::{Authorization, Bearer, Header, Raw};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use serde_json::{self, Value};

use super::super::i18n::{Lang, Locale};
use super::super::jwt::Jwt;
use super::super::orm::Connection as Db;
use super::super::result::Result;
use super::models::{Role, User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub uid: String,
    pub roles: Vec<String>,
}

pub struct CurrentUser {
    pub id: i64,
    pub roles: Vec<String>,
}

impl CurrentUser {
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&s!(Role::ADMIN))
    }
}

impl CurrentUser {
    fn get(jwt: &Jwt, db: &Db, lang: &String, token: &String) -> Result<CurrentUser> {
        let token = jwt.parse(token)?;
        let ss = serde_json::from_value::<Session>(token)?;
        let it = User::get_by_uid(db, &ss.uid)?;
        let roles = User::roles(db, &it.id)?;
        match it.status() {
            Some(code) => Err(Locale::e(db, &lang, &code, None::<Value>)),
            None => Ok(CurrentUser {
                id: it.id,
                roles: roles,
            }),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if let Some(auth) = req.headers()
            .get_one(Authorization::<Bearer>::header_name())
        {
            if let Ok(auth) = Authorization::<Bearer>::parse_header(&Raw::from(auth)) {
                let Authorization::<Bearer>(bearer) = auth;
                let jwt = req.guard::<State<Jwt>>()?;
                let db = req.guard::<Db>()?;
                let lang = req.guard::<Lang>()?;
                let Lang(lang) = lang;
                if let Ok(it) = CurrentUser::get(&jwt, &db, &lang, &bearer.token) {
                    return Outcome::Success(it);
                }
            }
        }
        Outcome::Failure((Status::Forbidden, ()))
    }
}

pub struct AdminUser {
    pub id: i64,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let it = req.guard::<CurrentUser>()?;
        if it.is_admin() {
            return Outcome::Success(AdminUser { id: it.id });
        }
        Outcome::Failure((Status::Forbidden, ()))
    }
}
