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
use crate::bool_util::str_to_bool;
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use std::env::{remove_var, set_var, var, VarError};

pub const ISOPY_CACHE_DIR_ENV_NAME: &str = "ISOPY_CACHE_DIR";
pub const ISOPY_OFFLINE_ENV_NAME: &str = "ISOPY_OFFLINE";
pub const ISOPY_BACKTRACE_ENV_NAME: &str = "ISOPY_BACKTRACE";
pub const ISOPY_LOG_LEVEL_ENV_NAME: &str = "ISOPY_LOG_LEVEL";
pub const ISOPY_BYPASS_ENV_ENV_NAME: &str = "ISOPY_BYPASS_ENV";
pub const RUST_BACKTRACE_ENV_NAME: &str = "RUST_BACKTRACE";

const BOOL_TRUE_VALUE: &str = "true";
const BOOL_FALSE_VALUE: &str = "false";

lazy_static! {
    pub static ref ENVS: Vec<(&'static str, EnvVarType)> = vec![
        (ISOPY_CACHE_DIR_ENV_NAME, EnvVarType::Ignore),
        (ISOPY_OFFLINE_ENV_NAME, EnvVarType::Bool),
        (ISOPY_BACKTRACE_ENV_NAME, EnvVarType::Bool),
        (ISOPY_LOG_LEVEL_ENV_NAME, EnvVarType::Ignore),
        (ISOPY_BYPASS_ENV_ENV_NAME, EnvVarType::Bool),
        (RUST_BACKTRACE_ENV_NAME, EnvVarType::Ignore)
    ];
}

pub enum EnvVarType {
    Ignore,
    Bool,
}

impl EnvVarType {
    fn normalize_all(envs: &[(&str, Self)]) -> Result<()> {
        for (key, t) in envs.iter() {
            match var(key) {
                Ok(value) => t.normalize(key, &value),
                Err(VarError::NotPresent) => {}
                Err(e) => bail!(e),
            };
        }
        Ok(())
    }

    fn normalize(&self, key: &str, value: &str) {
        match self {
            Self::Ignore => {}
            Self::Bool => match str_to_bool(value) {
                Some(true) => set_var(key, BOOL_TRUE_VALUE),
                Some(false) => set_var(key, BOOL_FALSE_VALUE),
                None => remove_var(key),
            },
        }
    }
}

pub fn get_env_bool(key: &str) -> bool {
    var(key) == Ok(String::from(BOOL_TRUE_VALUE))
}

pub fn normalize_all_envs() -> Result<()> {
    EnvVarType::normalize_all(&ENVS)
}
