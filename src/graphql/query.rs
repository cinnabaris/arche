use juniper::{self, FieldResult};

use super::super::{env::VERSION, plugins::nut};
use super::{context::Context, H};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        VERSION
    }

    //--------------------nut-----------------------
    field listLocalesByLang(&executor, lang: String) -> FieldResult<Vec<nut::graphql::models::Locale>> {
        gq!(executor, nut::graphql::query::ListLocaleByLang{lang: lang})
    }
    field listLog(&executor) -> FieldResult<Vec<nut::graphql::models::Log>> {
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
