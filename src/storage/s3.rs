#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub bucket: String,
    pub region: String,
}

pub struct Storage {}

impl super::Provider for Storage {}
