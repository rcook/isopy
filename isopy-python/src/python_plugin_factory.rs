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
use crate::constants::{PYTHON_BIN_FILE_NAME, PYTHON_SCRIPT_EXT};
use crate::serialization::ProjectConfig;
use isopy_lib::{
    other_error as isopy_lib_other_error, render_absolute_path, Descriptor, IsopyLibResult,
    Platform, PluginFactory, Shell,
};
use serde_json::Value;
use std::ffi::{OsStr, OsString};
use std::path::Path;

pub struct PythonPluginFactory;

impl Default for PythonPluginFactory {
    fn default() -> Self {
        Self
    }
}

impl PluginFactory for PythonPluginFactory {
    fn read_project_config(&self, props: &Value) -> IsopyLibResult<Box<dyn Descriptor>> {
        let project_config = serde_json::from_value::<ProjectConfig>(props.clone())
            .map_err(isopy_lib_other_error)?;
        Ok(Box::new(project_config.descriptor))
    }

    fn make_script_command(
        &self,
        script_path: &Path,
        _platform: Platform,
        shell: Shell,
    ) -> IsopyLibResult<Option<OsString>> {
        fn make_command(script_path: &Path, shell: Shell) -> IsopyLibResult<OsString> {
            let delimiter: &str = match shell {
                Shell::Bash => "'",
                Shell::Cmd => "\"",
            };

            let mut s = OsString::new();
            s.push(PYTHON_BIN_FILE_NAME.as_os_str());
            s.push(" ");
            s.push(delimiter);
            s.push(render_absolute_path(shell, script_path)?);
            s.push(delimiter);
            Ok(s)
        }

        if script_path
            .extension()
            .map(OsStr::to_ascii_lowercase)
            .as_ref()
            == Some(&*PYTHON_SCRIPT_EXT)
        {
            Ok(Some(make_command(script_path, shell)?))
        } else {
            Ok(None)
        }
    }
}
