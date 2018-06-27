#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "endpoint")]
    pub end_point: String,
    #[serde(rename = "localroot")]
    pub local_root: String,
}

impl Config {
    pub fn open(&self) -> Storage {
        Storage { _cfg: self.clone() }
    }
}

pub struct Storage {
    _cfg: Config,
}

impl super::Provider for Storage {}
