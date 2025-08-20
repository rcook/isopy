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
use anyhow::{Result, bail};
use std::env::{VarError, remove_var, set_var, var};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::sync::LazyLock;

pub(crate) const BOOL_TRUE_VALUE: &str = "true";
pub(crate) const BOOL_FALSE_VALUE: &str = "false";

#[derive(Clone, Copy)]
pub(crate) enum EnvKey {
    BacktraceEnabled,
    ConfigDir,
    LogLevel,
    JavaEnabled,
    RustBacktrace,
    IsopyEnv,
}

impl EnvKey {
    pub(crate) const fn name(&self) -> &str {
        match self {
            Self::BacktraceEnabled => "ISOPY_BACKTRACE",
            Self::ConfigDir => "ISOPY_CONFIG_DIR",
            Self::LogLevel => "ISOPY_LOG_LEVEL",
            Self::JavaEnabled => "ISOPY_JAVA",
            Self::RustBacktrace => "RUST_BACKTRACE",
            Self::IsopyEnv => "ISOPY_ENV",
        }
    }

    pub(crate) fn get(self) -> StdResult<String, VarError> {
        var(self.name())
    }

    pub(crate) fn set(self, s: &str) {
        unsafe { set_var(self.name(), s) };
    }

    pub(crate) fn is_present(self) -> bool {
        self.get() != Err(VarError::NotPresent)
    }

    pub(crate) fn is_true(self) -> bool {
        self.get() == Ok(String::from(BOOL_TRUE_VALUE))
    }
}

impl Display for EnvKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Copy)]
enum EnvType {
    Ignore,
    Bool,
}

static CLI_ENVS: LazyLock<Vec<(EnvKey, EnvType)>> = LazyLock::new(|| {
    vec![
        (EnvKey::BacktraceEnabled, EnvType::Bool),
        (EnvKey::ConfigDir, EnvType::Ignore),
        (EnvKey::LogLevel, EnvType::Ignore),
        (EnvKey::JavaEnabled, EnvType::Bool),
        (EnvKey::RustBacktrace, EnvType::Ignore),
        (EnvKey::IsopyEnv, EnvType::Ignore),
    ]
});

enum Op {
    DoNothing,
    SetVar(String),
    RemoveVar,
}

struct Action {
    key: String,
    op: Op,
}

impl Action {
    fn make_all(envs: &[(EnvKey, EnvType)]) -> Result<Vec<Self>> {
        fn make(env_key: EnvKey, env_type: EnvType, value: &str) -> Action {
            let key = env_key.to_string();
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

        envs.iter()
            .map(|(k, v)| {
                Ok(match read_env(*k)? {
                    Some(value) => make(*k, *v, &value),
                    None => Self {
                        key: k.to_string(),
                        op: Op::DoNothing,
                    },
                })
            })
            .collect::<Result<Vec<_>>>()
    }

    fn run(&self) {
        match &self.op {
            Op::DoNothing => {}
            Op::SetVar(value) => unsafe { set_var(&self.key, value) },
            Op::RemoveVar => unsafe { remove_var(&self.key) },
        }
    }
}

pub(crate) fn set_up_env() -> Result<()> {
    for action in Action::make_all(&CLI_ENVS)? {
        action.run();
    }

    if EnvKey::BacktraceEnabled.is_true() && !EnvKey::RustBacktrace.is_present() {
        EnvKey::RustBacktrace.set("1");
    }

    Ok(())
}

pub(crate) fn get_env_keys() -> Vec<EnvKey> {
    CLI_ENVS.iter().map(|(key, _)| *key).collect::<Vec<_>>()
}

pub(crate) fn read_env(key: EnvKey) -> Result<Option<String>> {
    match var(key.name()) {
        Ok(s) => Ok(Some(s)),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => bail!(e),
    }
}
