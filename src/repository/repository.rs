use crate::object_model::{Asset, LastModified};
use crate::result::Result;
use crate::util::ContentLength;
use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait Repository {
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<Box<dyn Response>>>;

    async fn get_asset(&self, asset: &Asset) -> Result<Box<dyn Response>>;
}

pub trait Response {
    fn last_modified(&self) -> &Option<LastModified>;
    fn content_length(&self) -> Option<ContentLength>;
    fn bytes_stream(&mut self) -> Result<Box<dyn Stream>>;
}

#[async_trait]
pub trait Stream {
    async fn next(&mut self) -> Option<Result<Bytes>>;
}
