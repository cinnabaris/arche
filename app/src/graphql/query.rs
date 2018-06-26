use juniper::{self, FieldResult};
use nut::{self, graphql::context::Context};

use super::super::app::VERSION;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    //--------------------nut-----------------------

    field locales(&executor, form: nut::graphql::query::ListLocaleByLang) -> FieldResult<Vec<nut::graphql::models::Locale>> {
        gq!(executor, form)
    }

    //--------------------forum--------------------

});
