pub mod schema;

use diesel::mysql::MysqlConnection;

pub type Connection = MysqlConnection;
