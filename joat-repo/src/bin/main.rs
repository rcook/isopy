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
#![warn(clippy::all)]
#![warn(clippy::cargo)]
//#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
//#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]
mod cli;

use crate::cli::{
    Args, Logger, Status, Subcommand, do_find, do_info, do_init, do_link, do_list, do_purge,
    do_read, do_remove, do_show, do_trash, do_write,
};
use anyhow::{Result, anyhow};
use clap::Parser;
use joat_repo::{Repo, RepoConfig};
use log::error;
use log::{LevelFilter, set_logger, set_max_level};
use path_absolutize::Absolutize;
use std::env::{VarError, current_dir, set_var, var};
use std::path::{Path, PathBuf};
use std::process::exit;

static LOGGER: Logger = Logger;

#[cfg(debug_assertions)]
fn init_backtrace() {
    const RUST_BACKTRACE_ENV_NAME: &str = "RUST_BACKTRACE";

    if var(RUST_BACKTRACE_ENV_NAME) == Err(VarError::NotPresent) {
        set_var(RUST_BACKTRACE_ENV_NAME, "1");
    }

    color_backtrace::install();
}

#[cfg(not(debug_assertions))]
fn init_backtrace() {}

fn init_logger() -> Result<()> {
    set_logger(&LOGGER).map_err(|e| anyhow!(e))?;
    set_max_level(LevelFilter::Info);
    Ok(())
}

fn get_repo_dir(cwd: &Path, args: &Args) -> Result<PathBuf> {
    Ok(if let Some(repo_dir) = &args.repo_dir {
        repo_dir.absolutize_from(cwd)?.to_path_buf()
    } else {
        let mut repo_dir = home::home_dir().ok_or_else(|| anyhow!("cannot get home directory"))?;
        repo_dir.push(".joat-repo-example-bin");
        repo_dir
    })
}

fn main() -> Result<()> {
    init_backtrace();
    init_logger()?;

    exit(match run()? {
        Status::Success => 0,
        Status::Failure => 1,
    });
}

fn run() -> Result<Status> {
    let args = Args::parse();
    let cwd = current_dir()?;
    let repo_dir = get_repo_dir(&cwd, &args)?;

    if let Some(repo) = RepoConfig::default(&repo_dir, None).repo()? {
        run_command(&args, &repo, &cwd)
    } else {
        error!(
            "Repository at {} is currently in use by another program or lock file is invalid",
            repo_dir.display()
        );
        Ok(Status::Failure)
    }
}

fn run_command(args: &Args, repo: &Repo, cwd: &Path) -> Result<Status> {
    match &args.subcommand {
        Subcommand::Find => do_find(repo, cwd),
        Subcommand::Info => Ok(do_info(repo)),
        Subcommand::Init => do_init(repo, cwd),
        Subcommand::Link { meta_id } => do_link(repo, meta_id, cwd),
        Subcommand::List => do_list(repo),
        Subcommand::Purge { force } => do_purge(repo, *force),
        Subcommand::Read { path } => do_read(repo, path),
        Subcommand::Remove => do_remove(repo, cwd),
        Subcommand::Show => do_show(repo, cwd),
        Subcommand::Trash { clean } => do_trash(repo, *clean),
        Subcommand::Write { path, value } => do_write(repo, path, value),
    }
}
