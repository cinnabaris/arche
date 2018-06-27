#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "endpoint")]
    pub end_point: String,
    #[serde(rename = "localroot")]
    pub local_root: String,
}

pub struct Storage {}

impl super::Provider for Storage {}
