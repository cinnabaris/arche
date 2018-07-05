#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nfs {
    #[serde(rename = "endpoint")]
    pub end_point: String,
    #[serde(rename = "localroot")]
    pub local_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3 {
    pub bucket: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub nfs: Option<Nfs>,
    pub s3: Option<S3>,
}

pub struct Storage {
    _cfg: Config,
}

impl Storage {
    pub fn new(cfg: Config) -> Self {
        Self { _cfg: cfg }
    }
}
