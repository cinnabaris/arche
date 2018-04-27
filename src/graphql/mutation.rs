use juniper::FieldResult;

use super::context::Context;

pub struct Mutation;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: i32,
    pub name: String,
}

graphql_object!(Mutation: Context |&self| {

    field createHuman(&executor, name: String) -> FieldResult<Human> {
        // let db = executor.context().pool.get_connection()?;
        // let human: Human = db.insert_human(&new_human)?;
        Ok(Human{id:123, name:format!("create name {}",name)})
    }
});
