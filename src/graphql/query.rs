use juniper::{self, FieldResult};

use super::super::{env, plugins::nut};
use super::{context::Context, H};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> String {
        env::version()
    }

    //--------------------nut-----------------------

    field getLocale(&executor, code: String) -> FieldResult<nut::graphql::locales::Locale> {
        gq!(executor, nut::graphql::locales::Get{code: code})
    }
    field listLocaleByLang(&executor, lang: String) -> FieldResult<Vec<nut::graphql::locales::Locale>> {
        gq!(executor, nut::graphql::locales::ListByLang{lang: lang})
    }

    field getUserProfile(&executor) -> FieldResult<nut::graphql::users::models::Profile> {
        ge!(nut::graphql::users::query::get_profile(executor.context()))
    }
    field listUserLog(&executor) -> FieldResult<Vec<nut::graphql::users::models::Log>> {
        ge!(nut::graphql::users::query::list_log(executor.context()))
    }
    field forgotUserPassword(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::users::query::ForgotPassword{
            email: email.to_lowercase(),
        })
    }
    field unlockUser(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::users::query::Unlock{
            email: email.to_lowercase(),
        })
    }
    field confirmUser(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::users::query::Confirm{
            email: email.to_lowercase(),
        })
    }

});
