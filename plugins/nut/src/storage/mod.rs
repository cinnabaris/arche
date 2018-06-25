pub mod nfs;
pub mod s3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub nfs: Option<nfs::Config>,
    pub s3: Option<s3::Config>,
}
