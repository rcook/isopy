// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::api::python_standalone_builds::LastModified;
use crate::download::ContentLength;
use crate::python::Asset;
use anyhow::Result;
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
