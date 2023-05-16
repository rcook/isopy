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
use crate::object_model::{EnvironmentName, Tag, Version};
use clap::{Args as ClapArgs, Parser, Subcommand};
use joat_repo::MetaId;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_HOME_PAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const PACKAGE_BUILD_VERSION: Option<&str> = option_env!("RUST_TOOL_ACTION_BUILD_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = PACKAGE_NAME,
    version = PACKAGE_VERSION,
    about = format!("{} {}", PACKAGE_DESCRIPTION, PACKAGE_VERSION),
    after_help = format!("{}\nhttps://github.com/rcook/isopy{}", PACKAGE_HOME_PAGE, PACKAGE_BUILD_VERSION.map(|x| format!("\n\n{}", x)).unwrap_or(String::from("")))
)]
pub struct Args {
    #[arg(global = true, help = "Path to isopy cache directory", long = "cache-dir", value_parser = parse_absolute_path)]
    pub cache_dir: Option<PathBuf>,

    #[arg(global = true, help = "Path to working directory", short = 'c', long = "cwd", value_parser = parse_absolute_path)]
    pub cwd: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(
        name = "available",
        about = "List Python packages available for download"
    )]
    Available,

    #[command(name = "download", about = "Download Python package")]
    Download(PythonVersion),

    #[command(name = "downloaded", about = "List downloaded Python packages")]
    Downloaded,

    #[command(name = "exec", about = "Execute command in Python environment")]
    Exec {
        #[arg(help = "Program to run in environment")]
        program: String,

        #[arg(help = "Zero or more arguments to pass to program")]
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    #[command(
        name = "gen-config",
        about = "Generate .python-version.yaml Python configuration file"
    )]
    GenConfig(PythonVersion),

    #[command(name = "info", about = "Show information")]
    Info,

    #[command(name = "init", about = "Create Python environment")]
    Init(PythonVersion),

    #[command(
        name = "init-config",
        about = "Create Python environment from .python-version.yaml configuration file"
    )]
    InitConfig,

    #[command(
        name = "link",
        about = "Use existing Python environment for current directory"
    )]
    Link {
        #[arg(help = "Meta ID", value_parser = parse_meta_id)]
        meta_id: MetaId,
    },

    #[command(name = "list", about = "List Python environments")]
    List,

    #[command(name = "shell", about = "Start Python environment shell")]
    Shell,

    #[command(name = "wrap", about = "Generate wrapper script for Python script")]
    Wrap {
        #[arg(help = "Wrapper path", value_parser = parse_absolute_path)]
        wrapper_path: PathBuf,

        #[arg(help = "Script path", value_parser = parse_absolute_path)]
        script_path: PathBuf,

        #[arg(help = "Base directory", value_parser = parse_absolute_path)]
        base_dir: PathBuf,
    },
}

#[derive(ClapArgs, Debug)]
pub struct PythonVersion {
    #[arg(help = "Python version", value_parser = parse_version)]
    pub version: Version,

    #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
    pub tag: Option<Tag>,
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}

fn parse_environment_name(s: &str) -> Result<EnvironmentName, String> {
    EnvironmentName::parse(s).ok_or(String::from("invalid environment name"))
}

fn parse_version(s: &str) -> Result<Version, String> {
    Version::parse(s).map_err(|_| String::from("invalid version"))
}

fn parse_tag(s: &str) -> Result<Tag, String> {
    Ok(Tag::parse(s))
}

fn parse_meta_id(s: &str) -> Result<MetaId, String> {
    MetaId::parse(s).ok_or(String::from("invalid meta ID"))
}
