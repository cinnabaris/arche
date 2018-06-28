use juniper::{self, FieldResult};

use super::super::{env::VERSION, plugins::nut};
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    //--------------------nut-----------------------
    field getSiteInfo(&executor) -> FieldResult<nut::graphql::models::SiteInfo> {
        ge!(nut::graphql::query::get_site_info(&executor.context()))
    }
    field listLocalesByLang(&executor, form: nut::graphql::query::ListLocaleByLang) -> FieldResult<Vec<nut::graphql::models::Locale>> {
        gq!(executor, form)
    }

    //--------------------forum--------------------

});