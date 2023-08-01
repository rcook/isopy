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

enum EnvType {
    Ignore,
    Bool,
}

lazy_static! {
    static ref ENVS: Vec<(&'static str, EnvType)> = vec![
        (ISOPY_CACHE_DIR_ENV_NAME, EnvType::Ignore),
        (ISOPY_OFFLINE_ENV_NAME, EnvType::Bool),
        (ISOPY_BACKTRACE_ENV_NAME, EnvType::Bool),
        (ISOPY_LOG_LEVEL_ENV_NAME, EnvType::Ignore),
        (ISOPY_BYPASS_ENV_ENV_NAME, EnvType::Bool),
        (RUST_BACKTRACE_ENV_NAME, EnvType::Ignore)
    ];
}

#[derive(Debug)]
enum Op {
    DoNothing,
    SetVar(String),
    RemoveVar,
}

#[derive(Debug)]
struct Action {
    key: String,
    op: Op,
}

impl Action {
    fn make_all(envs: &[(&str, EnvType)]) -> Result<Vec<Self>> {
        fn make(env_type: &EnvType, key: &str, value: &str) -> Action {
            let key = String::from(key);
            match env_type {
                EnvType::Ignore => Action {
                    key,
                    op: Op::DoNothing,
                },
                EnvType::Bool => match str_to_bool(value) {
                    Some(true) => Action {
                        key,
                        op: Op::SetVar(String::from(BOOL_TRUE_VALUE)),
                    },
                    Some(false) => Action {
                        key,
                        op: Op::SetVar(String::from(BOOL_FALSE_VALUE)),
                    },
                    None => Action {
                        key,
                        op: Op::RemoveVar,
                    },
                },
            }
        }

        let mut actions = Vec::new();
        for (key, t) in envs {
            let action = match read_env_var(key)? {
                Some(value) => make(t, key, &value),
                None => Self {
                    key: String::from(*key),
                    op: Op::DoNothing,
                },
            };
            actions.push(action);
        }

        Ok(actions)
    }

    fn run(&self) {
        match &self.op {
            Op::DoNothing => {}
            Op::SetVar(value) => set_var(&self.key, value),
            Op::RemoveVar => remove_var(&self.key),
        }
    }
}

pub fn transform_env_vars() -> Result<()> {
    for action in Action::make_all(&ENVS)? {
        action.run();
    }
    Ok(())
}

pub fn get_env_keys() -> Vec<String> {
    ENVS.iter().map(|e| String::from(e.0)).collect::<Vec<_>>()
}

pub fn read_env_var(key: &str) -> Result<Option<String>> {
    match var(key) {
        Ok(s) => Ok(Some(s)),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => bail!(e),
    }
}

pub fn read_env_var_bool(key: &str) -> bool {
    var(key) == Ok(String::from(BOOL_TRUE_VALUE))
}
