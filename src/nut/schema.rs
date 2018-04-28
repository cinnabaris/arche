#[derive(Serialize, GraphQLObject, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}
