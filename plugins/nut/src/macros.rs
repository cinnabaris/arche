#[macro_export]
macro_rules! db {
    ($x:expr, $y:expr) => {
        match $x {
            ::orm::Connection::PostgreSql(db) => {
                use orm::postgresql::schema::*;
                $y(db)
            }
            _ => Err("not supported database".into()),
        }
    };
}

#[macro_export]
macro_rules! cache {
    ($x:expr, $y:expr) => {
        match $x {
            ::cache::Cache::Redis(ch) => $y(ch),
            _ => Err("not supported cache".into()),
        }
    };
}
