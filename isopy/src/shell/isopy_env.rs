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
use crate::env::EnvKey;
use anyhow::{Result, bail};
use joat_repo::{DirInfo, LinkId, MetaId};
use std::env::VarError;

#[derive(Clone, Debug)]
pub(crate) struct IsopyEnv {
    pub(crate) link_id: LinkId,
    pub(crate) meta_id: MetaId,
}

impl IsopyEnv {
    pub(crate) fn from_dir_info(dir_info: &DirInfo) -> Self {
        Self {
            link_id: dir_info.link_id().clone(),
            meta_id: dir_info.meta_id().clone(),
        }
    }

    pub(crate) fn get_vars() -> Result<Option<Self>> {
        let value = match EnvKey::IsopyEnv.get() {
            Ok(value) => value,
            Err(VarError::NotPresent) => return Ok(None),
            Err(e) => return Err(e)?,
        };

        let Some((prefix, suffix)) = value.split_once('-') else {
            bail!("invalid value for {name}: {value}", name = EnvKey::IsopyEnv);
        };

        let meta_id = prefix.parse::<MetaId>()?;
        let link_id = suffix.parse::<LinkId>()?;

        Ok(Some(Self { link_id, meta_id }))
    }

    pub(crate) fn set_vars(&self) {
        EnvKey::IsopyEnv.set(&format!("{}-{}", self.meta_id, self.link_id));
    }
}
