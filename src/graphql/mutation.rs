use juniper::{self, FieldResult};

use super::super::plugins::nut;
use super::{context::Context, H};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field removeLocale(&executor, id: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::locales::Remove{
                id: id,
            })
        }
        field updateLocale(&executor, code: String, message: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::locales::Update{
                code: code,
                message: message,
            })
        }

        field createLeaveWord(&executor, media_type: String, body: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::leave_words::Create{
                media_type: media_type,
                body: body,
            })
        }

        field updateUserProfile(&executor, name: String, logo: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::UpdateProfile{
                name: name,
                logo: logo,
            })
        }
        field changeUserPassword(&executor, current_password: String, new_password: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::ChangePassword{
                current_password: current_password,
                new_password: new_password,
            })
        }
        field signOutUser(&executor) -> FieldResult<H> {
            ge!(nut::graphql::users::mutation::sign_out(executor.context()))
        }

        field signInUserByEmail(&executor, email: String, password: String) -> FieldResult<nut::graphql::users::models::SignIn> {
            gq!(executor, nut::graphql::users::mutation::SignInByEmail{
                email: email,
                password: password,
            })
        }
        field resetUserPassword(&executor, token: String, password: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::ResetPassword{
                token: token,
                password: password,
            })
        }
        field unlockUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::Unlock{
                token: token,
            })
        }
        field confirmUser(&executor, token: String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::Confirm{
                token: token,
            })
        }
        field signUpUser(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::users::mutation::SignUp{
                name: name,
                email: email.to_lowercase(),
                password: password,
            })
        }

        field install(&executor, name: String, email:String, password:String) -> FieldResult<H> {
            gq!(executor, nut::graphql::home::Install{
                name: name,
                email: email.to_lowercase(),
                password: password,
            })
        }
    }
);
