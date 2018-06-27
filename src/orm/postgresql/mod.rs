pub mod schema;

use diesel::pg::PgConnection;

// logging:
// edit "/var/lib/postgres/data/postgresql.conf", change "log_statement = 'all'"
// sudo gpasswd -a YOUR-NAME wheel
// journalctl -f -u postgresql
// show database size: /l+
// \c arche
// \d xxx
pub type Connection = PgConnection;
