use juniper::{self, FieldResult};

use super::super::plugins::nut;
use super::{context::Context, H};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field updateLocale(&executor, form: nut::graphql::mutation::UpdateLocale) -> FieldResult<H> {
            gq!(executor, form)
        }
        field signUpUser(&executor, form: nut::graphql::mutation::SignUpUser) -> FieldResult<H> {
            gq!(executor, form)
        }
        field install(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::Install{
                name: name,
                email: email,
                password: password,
            })
        }
    }
);
