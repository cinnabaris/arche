use juniper::FieldResult;

use super::super::env::VERSION;
use super::super::nut;
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    field locales(&executor, lang: String) -> FieldResult<nut::schema::User> {

        // let context = executor.context();
        // let connection = context.pool.get_connection()?;
        // let human = connection.find_human(&id)?;
        Ok(nut::schema::User{id:s!("aaa"), name:s!("bbb")})
    }
});
