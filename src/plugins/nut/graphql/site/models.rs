use super::super::super::super::super::oauth::{self, Oauth as OauthUrl};

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Smtp {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Seo {
    pub google: String,
    pub baidu: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Status {
    pub name: String,
    pub value: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Info {
    pub title: String,
    pub subhead: String,
    pub keywords: String,
    pub description: String,
    pub copyright: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Oauth {
    pub name: String,
    pub url: String,
}
impl Oauth {
    pub fn new(cfg: &oauth::Config) -> Vec<Self> {
        let mut items = Vec::new();
        if let Some(ref c) = cfg.line {
            items.push(Self {
                name: c.name().to_string(),
                // TODO
                url: c.authorization_url(&"".to_string(), &"".to_string()),
            });
        }
        items
    }
}
