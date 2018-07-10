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
        field signOutUser(&executor) -> FieldResult<H> {
            ge!(nut::graphql::mutation::sign_out_user(executor.context()))
        }
        field createLeaveWord(&executor, media_type: String, body: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::CreateLeaveWord{
                media_type: media_type,
                body: body,
            })
        }
        field signInUserByEmail(&executor, email: String, password: String) -> FieldResult<nut::graphql::models::SignIn> {
            gq!(executor, nut::graphql::mutation::SignInUserByEmail{
                email: email,
                password: password,
            })
        }
        field resetUserPassword(&executor, token: String, password: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::ResetUserPassword{
                token: token,
                password: password,
            })
        }
        field unlockUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::UnlockUser{
                token: token,
            })
        }
        field confirmUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::mutation::ConfirmUser{
                token: token,
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
