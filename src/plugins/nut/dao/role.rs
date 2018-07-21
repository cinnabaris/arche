use std::fmt;
use std::str::FromStr;

use super::super::super::super::errors::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    Root,
    Admin,
    Manager,
    Member,
    By(String),
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Root => write!(fmt, "root"),
            Type::Admin => write!(fmt, "admin"),
            Type::Manager => write!(fmt, "manager"),
            Type::Member => write!(fmt, "member"),
            Type::By(n) => write!(fmt, "{}", &n),
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "root" => Type::Root,
            "admin" => Type::Admin,
            "manager" => Type::Manager,
            "member" => Type::Member,
            v => Type::By(v.to_string()),
        })
    }
}
