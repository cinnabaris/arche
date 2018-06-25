use super::super::errors::Result;

pub trait Handler: Send + Sync {
    fn handle(
        &self,
        id: &String,
        type_: &String,
        content_type: &String,
        priority: u8,
        payload: &[u8],
    ) -> Result<()>;
}
