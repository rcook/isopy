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
use crate::IsopyLibResult;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ProjectConfigInfo {
    pub value: serde_json::Value,
}

pub trait Descriptor: Any + Debug + Display + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn transform_archive_path(&self, path: &Path) -> PathBuf;

    fn get_env_config_value(&self) -> IsopyLibResult<serde_json::Value>;

    fn get_project_config_info(&self) -> IsopyLibResult<ProjectConfigInfo>;
}
