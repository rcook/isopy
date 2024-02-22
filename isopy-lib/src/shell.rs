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
use lazy_static::lazy_static;
use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, Prefix};

lazy_static! {
    static ref COLON_PATH_SEPARATOR: OsString = OsString::from(":");
    static ref SEMICOLON_PATH_SEPARATOR: OsString = OsString::from(";");
}

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

#[must_use] pub fn env_var_substitution(shell: Shell, env_var: &str) -> OsString {
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
        Shell::Bash => &COLON_PATH_SEPARATOR,
        Shell::Cmd => &SEMICOLON_PATH_SEPARATOR,
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
pub fn render_path(shell: Shell, path: &Path) -> OsString {
    match shell {
        Shell::Bash => OsString::from(path),
        Shell::Cmd => todo!("Will this ever be needed?"),
    }
}

// What an hideous and hideously inefficient mess
// It panics too!
// TBD: Fix this!
#[allow(clippy::missing_panics_doc)]
#[cfg(target_os = "windows")]
#[must_use] pub fn render_path(shell: Shell, path: &Path) -> OsString {
    match shell {
        Shell::Bash => {
            let mut s = OsString::new();
            let mut iter = path.components();
            match iter.next() {
                Some(Component::Prefix(prefix)) => match prefix.kind() {
                    Prefix::Disk(raw) => {
                        s.push("/");
                        let temp = String::from_utf8(vec![raw]).expect("unimplemented");
                        s.push(temp);
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            match iter.next() {
                Some(Component::RootDir) => {}
                _ => unimplemented!(),
            }
            for component in iter {
                s.push("/");
                s.push(component);
            }
            s
        }
        Shell::Cmd => OsString::from(path),
    }
}
