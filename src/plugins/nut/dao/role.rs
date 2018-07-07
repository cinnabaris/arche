use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    Admin,
    Root,
    Member,
    By(String),
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Admin => write!(fmt, "admin"),
            Type::Root => write!(fmt, "root"),
            Type::Member => write!(fmt, "member"),
            Type::By(n) => write!(fmt, "{}", &n),
        }
    }
}
