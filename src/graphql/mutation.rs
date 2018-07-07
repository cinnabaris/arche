use juniper::{self, FieldResult};

use super::super::plugins::nut;
use super::{context::Context, H};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field updateLocale(&executor, code: String, message: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::UpdateLocale{
                code: code,
                message: message,
            })
        }
        field signUpUser(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::SignUpUser{
                name: name,
                email: email.to_lowercase(),
                password: password,
            })
        }
        field install(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::Install{
                name: name,
                email: email.to_lowercase(),
                password: password,
            })
        }
    }
);
