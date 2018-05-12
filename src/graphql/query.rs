use juniper::{self, FieldResult};

use super::super::{
    env, plugins::{cbeta, nut},
};
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        env::VERSION
    }

    //--------------------nut--------------------

    field locales(&executor, form: nut::schema::locales::FmByLang) -> FieldResult<Vec<nut::schema::locales::Locale>> {
        gq!(executor, form)
    }

    //--------------------cbeta--------------------

    field cbetaIndexBook(&executor) -> FieldResult<Vec<cbeta::schema::books::Book>> {
        ge!(cbeta::schema::books::all(&executor.context()))
    }

    field cbetaShowBook(&executor, form: cbeta::schema::books::FmById) -> FieldResult<cbeta::models::Book> {
        gq!(executor, form)
    }

});
