use juniper::{self, FieldResult};

use super::context::Context;

use super::super::nut;

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field usersSignUp(&executor, form: nut::forms::UsersSignUp) -> FieldResult<nut::schema::User> {
        gq!(executor, form)
    }

});
