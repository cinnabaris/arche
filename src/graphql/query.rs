use juniper::{self, FieldResult};

use super::super::env;
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        env::VERSION
    }

    // field locales(&executor, lang: String) -> FieldResult<Vec<i18n::Model>> {
    //     ge!(i18n::Model::by_lang(&executor.context().db, &lang))
    // }

});
