use juniper::{self, FieldResult};

use super::super::{env, plugins::nut};
use super::{context::Context, H};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> String {
        env::version()
    }

    //--------------------nut-----------------------
    field listLeaveWord(&executor) -> FieldResult<Vec<nut::graphql::leave_words::LeaveWord>> {
        ge!(nut::graphql::leave_words::list(executor.context()))
    }

    field getSiteSmtp(&executor) -> FieldResult<nut::graphql::site::models::Smtp> {
        ge!(nut::graphql::site::query::smtp(executor.context()))
    }
    field getSiteSeo(&executor) -> FieldResult<nut::graphql::site::models::Seo> {
        ge!(nut::graphql::site::query::seo(executor.context()))
    }
    field getSiteAuthor(&executor) -> FieldResult<nut::graphql::site::models::Author> {
        ge!(nut::graphql::site::query::author(executor.context()))
    }
    field getSiteInfo(&executor) -> FieldResult<nut::graphql::site::models::Info> {
        ge!(nut::graphql::site::query::info(executor.context()))
    }
    field getSiteStatus(&executor) -> FieldResult<Vec<nut::graphql::site::models::Status>> {
        ge!(nut::graphql::site::query::status(executor.context()))
    }

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
