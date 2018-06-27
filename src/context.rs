use super::orm;

pub struct Context {
    pub db: orm::Connection,
}
