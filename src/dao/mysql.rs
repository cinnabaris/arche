#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn url(&self) -> String {
        format!(
            "mysql://{user}:{password}@{host}:{port}/{name}",
            user = self.user,
            password = self.password,
            name = self.name,
            host = self.host,
            port = self.port,
        )
    }
}
