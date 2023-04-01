use crate::object_model::LastModified;
use crate::result::Result;
use crate::util::ContentLength;
use async_trait::async_trait;
use bytes::Bytes;

pub type ResponseInfo = (LastModified, Option<ContentLength>, Box<dyn Response>);

#[async_trait]
pub trait Repository {
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<ResponseInfo>>;
}

pub trait Response {
    fn content_length(&self) -> Option<ContentLength>;
    fn bytes_stream(&mut self) -> Result<Box<dyn Stream>>;
}

#[async_trait]
pub trait Stream {
    async fn next(&mut self) -> Option<Result<Bytes>>;
}
