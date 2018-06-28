use juniper::{self, FieldResult};

use super::super::plugins::nut;
use super::context::Context;

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field updateLocale(&executor, form: nut::graphql::mutation::UpdateLocale) -> FieldResult<String> {
            gq!(executor, form)
        }
    }
);
