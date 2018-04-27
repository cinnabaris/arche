use juniper::FieldResult;

use super::context::Context;
use super::mutation::Human;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field human(&executor, name: String) -> FieldResult<Human> {

        let context = executor.context();
        // let connection = context.pool.get_connection()?;
        // let human = connection.find_human(&id)?;
        Ok(Human{name:name,id:432})
    }
});
