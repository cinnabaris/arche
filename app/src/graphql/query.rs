use juniper::{self, FieldResult};
use nut::{self, graphql::context::Context};

use super::super::app::VERSION;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    //--------------------nut-----------------------
    field getSiteInfo(&executor) -> FieldResult<Result<nut::graphql::models::SiteInfo>> {
        ge!(nut::graphql::query::get_site_info(&executor.context()))
    }
    field listLocalesByLang(&executor, form: nut::graphql::query::ListLocaleByLang) -> FieldResult<Vec<nut::graphql::models::Locale>> {
        gq!(executor, form)
    }

    //--------------------forum--------------------

});
