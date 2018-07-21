use juniper::{self, FieldResult};

use super::super::{
    env,
    plugins::{forum, nut},
};
use super::{context::Context, H};

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> String {
        env::version()
    }

    //--------------------forum---------------------
    field showForumPost(&executor, id: String) -> FieldResult<forum::graphql::posts::Post> {
        gq!(executor, forum::graphql::posts::Show{id: id})
    }
    field listForumPost(&executor) -> FieldResult<Vec<forum::graphql::posts::Post>> {
        ge!(forum::graphql::posts::list(executor.context()))
    }
    field showForumTopic(&executor, id: String) -> FieldResult<forum::graphql::topics::Topic> {
        gq!(executor, forum::graphql::topics::Show{id: id})
    }
    field listForumTopic(&executor) -> FieldResult<Vec<forum::graphql::topics::Topic>> {
        ge!(forum::graphql::topics::list(executor.context()))
    }
    field showForumTag(&executor, id: String) -> FieldResult<forum::graphql::tags::Tag> {
        gq!(executor, forum::graphql::tags::Show{id: id})
    }
    field listForumTag(&executor) -> FieldResult<Vec<forum::graphql::tags::Tag>> {
        ge!(forum::graphql::tags::list(executor.context()))
    }

    //--------------------nut-----------------------
    field showFriendLink(&executor, id: String) -> FieldResult<nut::graphql::friend_links::FriendLink> {
        gq!(executor, nut::graphql::friend_links::Show{id: id})
    }
    field listFriendLink(&executor) -> FieldResult<Vec<nut::graphql::friend_links::FriendLink>> {
        ge!(nut::graphql::friend_links::list(executor.context()))
    }

    field showCard(&executor, id: String) -> FieldResult<nut::graphql::cards::Card> {
        gq!(executor, nut::graphql::cards::Show{id: id})
    }
    field listCard(&executor) -> FieldResult<Vec<nut::graphql::cards::Card>> {
        ge!(nut::graphql::cards::list(executor.context()))
    }

    field showLink(&executor, id: String) -> FieldResult<nut::graphql::links::Link> {
        gq!(executor, nut::graphql::links::Show{id: id})
    }
    field listLink(&executor) -> FieldResult<Vec<nut::graphql::links::Link>> {
        ge!(nut::graphql::links::list(executor.context()))
    }

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

    field getUserPolicy(&executor, id: String) -> FieldResult<Vec<nut::graphql::users::models::Policy>> {
        gq!(executor, nut::graphql::users::query::GetPolicy{
            id: id,
        })
    }
    field listUser(&executor) -> FieldResult<Vec<nut::graphql::users::models::User>> {
        ge!(nut::graphql::users::query::list(executor.context()))
    }
    field getUserProfile(&executor) -> FieldResult<nut::graphql::users::models::Profile> {
        ge!(nut::graphql::users::query::profile(executor.context()))
    }
    field listUserLog(&executor) -> FieldResult<Vec<nut::graphql::users::models::Log>> {
        ge!(nut::graphql::users::query::logs(executor.context()))
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
