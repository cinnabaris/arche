use juniper::{self, FieldResult};
use nut::{self, graphql::context::Context};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field updateLocale(&executor, form: nut::graphql::mutation::UpdateLocale) -> FieldResult<Option<String>> {
            gq!(executor, form)
        }
    }
);
