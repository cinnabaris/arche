use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    Root,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => write!(fmt, "admin"),
            Role::Root => write!(fmt, "root"),
            Role::Member => write!(fmt, "member"),
            Role::By(n) => write!(fmt, "{}", &n),
        }
    }
}
