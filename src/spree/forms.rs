use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserSignUp {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}
