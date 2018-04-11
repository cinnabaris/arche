use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct FmInstall {
    #[validate(length(min = "2", max = "32"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}
