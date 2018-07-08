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
        field forgotUserPassword(&executor, email:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::ForgotUserPassword{
                email: email.to_lowercase(),
            })
        }
        field unlockUser(&executor, email:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::UnlockUser{
                email: email.to_lowercase(),
            })
        }
        field confirmUser(&executor, email:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::ConfirmUser{
                email: email.to_lowercase(),
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
