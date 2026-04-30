// Copyright (c) 2026 Richard Cook
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
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn isopy() -> Command {
    Command::cargo_bin("isopy").unwrap()
}

fn isopy_with_dirs(config_dir: &TempDir, cwd: &TempDir) -> Command {
    let mut cmd = isopy();
    cmd.arg("--config-dir")
        .arg(config_dir.path())
        .arg("--cwd")
        .arg(cwd.path())
        .arg("--no-show-progress");
    cmd
}

#[test]
fn help_flag() {
    isopy()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Isolated Python Tool"));
}

#[test]
fn version_flag() {
    isopy()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn no_subcommand_shows_usage() {
    isopy()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn info_with_empty_repo() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("info")
        .assert()
        .success();
}

#[test]
fn list_with_empty_repo() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("ls")
        .assert()
        .success();
}

#[test]
fn list_brief_with_empty_repo() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("ls")
        .arg("--no-verbose")
        .assert()
        .success();
}

#[test]
fn check_with_empty_repo() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("check")
        .assert()
        .success();
}

#[test]
fn prompt_no_output_without_env() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("prompt")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn prompt_with_before_after() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("prompt")
        .arg("--before")
        .arg("[")
        .arg("--after")
        .arg("]")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn set_config_invalid_name() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("set-config")
        .arg("nonexistent_key")
        .arg("value")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("No such configuration value"));
}

#[test]
fn set_config_and_clear() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("set-config")
        .arg("default_moniker")
        .arg("python")
        .assert()
        .success();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("set-config")
        .arg("default_moniker")
        .assert()
        .success();
}

#[test]
fn invalid_subcommand() {
    isopy()
        .arg("not-a-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn check_with_clean_on_empty_repo() {
    let config_dir = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();

    isopy_with_dirs(&config_dir, &cwd)
        .arg("check")
        .arg("--clean")
        .assert()
        .success();
}
