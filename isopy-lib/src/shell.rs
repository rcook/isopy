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
use anyhow::Result;
use std::ffi::{OsStr, OsString};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
}

#[derive(Clone, Copy, Debug)]
pub enum Shell {
    Bash,
    Cmd,
}

#[must_use]
pub fn env_var_substitution(shell: Shell, env_var: &str) -> OsString {
    let mut s = OsString::new();
    match shell {
        Shell::Bash => {
            s.push("$");
            s.push(env_var);
        }
        Shell::Cmd => {
            s.push("%");
            s.push(env_var);
            s.push("%");
        }
    }
    s
}

pub fn path_separator(shell: Shell) -> &'static OsStr {
    match shell {
        Shell::Bash => OsStr::new(":"),
        Shell::Cmd => OsStr::new(";"),
    }
}

pub fn join_paths<I, T>(shell: Shell, paths: I) -> OsString
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    let sep = path_separator(shell);
    let mut s = OsString::new();
    for (i, path) in paths.enumerate() {
        if i > 0 {
            s.push(sep);
        }
        s.push(path);
    }
    s
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn render_absolute_path(_shell: Shell, path: &Path) -> Result<OsString> {
    Ok(OsString::from(path))
}

#[cfg(target_os = "windows")]
pub fn render_absolute_path(shell: Shell, path: &Path) -> Result<OsString> {
    use anyhow::bail;

    if !path.is_absolute() {
        bail!("Path {} is not absolute", path.display())
    }

    match shell {
        Shell::Bash => render_absolute_path_windows_bash(path),
        Shell::Cmd => Ok(OsString::from(path)),
    }
}

#[cfg(target_os = "windows")]
fn render_absolute_path_windows_bash(path: &Path) -> Result<OsString> {
    use anyhow::bail;
    use std::path::{Component, Prefix};

    let mut iter = path.components();

    let Some(Component::Prefix(prefix)) = iter.next() else {
        bail!("Path {} does not have a prefix component", path.display())
    };

    let Prefix::Disk(raw) = prefix.kind() else {
        bail!("Path {} does not have a disc component", path.display())
    };

    let mut s = OsString::new();
    s.push("/");
    let temp = String::from_utf8(vec![raw]).expect("unimplemented");
    s.push(temp);

    let Some(Component::RootDir) = iter.next() else {
        bail!(
            "Path {} does not have root directory component",
            path.display()
        )
    };

    for component in iter {
        s.push("/");
        s.push(component);
    }

    Ok(s)
}
