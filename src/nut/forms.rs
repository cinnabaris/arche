use validator::Validate;

use super::super::graphql::context::Context;
use super::super::result::Result;
use super::schema;

#[derive(Serialize, GraphQLInputObject, Deserialize, Debug, Validate)]
#[graphql(description = "Create a new account")]
pub struct UsersSignUp {
    #[validate(length(min = "2", max = "64"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl UsersSignUp {
    pub fn call(&self, _ctx: &Context) -> Result<schema::User> {
        self.validate()?;
        Ok(schema::User {
            id: s!("123"),
            name: s!("aaa"),
        })
    }
}
