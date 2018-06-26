// use juniper::{self, FieldResult};

// use super::super::plugins::nut;
use nut::graphql::context::Context;

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {

        // field usersSignUp(&executor, form: nut::schema::users::FmSignUp) -> FieldResult<H> {
        //     gq!(executor, form)
        // }

    }
);
