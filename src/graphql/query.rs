use juniper::{self, FieldResult};

use super::super::{env::VERSION, plugins::nut};
use super::{context::Context, H};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    //--------------------nut-----------------------
    field getLocale(&executor, code: String) -> FieldResult<nut::graphql::models::Locale> {
        gq!(executor, nut::graphql::query::GetLocale{code: code})
    }
    field listLocaleByLang(&executor, lang: String) -> FieldResult<Vec<nut::graphql::models::Locale>> {
        gq!(executor, nut::graphql::query::ListLocaleByLang{lang: lang})
    }
    field getUserProfile(&executor) -> FieldResult<nut::graphql::models::Profile> {
        ge!(nut::graphql::query::get_user_profile(executor.context()))
    }
    field listUserLog(&executor) -> FieldResult<Vec<nut::graphql::models::Log>> {
        ge!(nut::graphql::query::list_log(executor.context()))
    }
    field forgotUserPassword(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::query::ForgotUserPassword{
            email: email.to_lowercase(),
        })
    }
    field unlockUser(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::query::UnlockUser{
            email: email.to_lowercase(),
        })
    }
    field confirmUser(&executor, email:String) -> FieldResult<H> {
        gq!(executor, nut::graphql::query::ConfirmUser{
            email: email.to_lowercase(),
        })
    }

    //--------------------forum--------------------

});
