use juniper::{self, FieldResult};

use super::super::{env, plugins::nut};
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        env::VERSION
    }

    field locales(&executor, form: nut::schema::Locales) -> FieldResult<Vec<nut::schema::LocalesOut>> {
        gq!(executor, form)
    }

});
