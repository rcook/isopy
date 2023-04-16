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
use clap::{Parser, Subcommand};
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
    #[arg(global = true, help = "Path to isopy cache directory", short = 'd', long = "dir", value_parser = parse_absolute_path)]
    pub dir: Option<PathBuf>,
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

    #[command(name = "create", about = "Create named Python environment")]
    Create {
        #[arg(help = "Environment name", value_parser = parse_environment_name)]
        environment_name: EnvironmentName,
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },

    #[command(name = "download", about = "Download Python package")]
    Download {
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },

    #[command(name = "downloaded", about = "List downloaded Python package")]
    Downloaded,

    #[command(name = "exec", about = "List downloaded Python package")]
    Exec {
        #[arg(help = "Environment name", short = 'e', long = "env", value_parser = parse_environment_name)]
        environment_name: Option<EnvironmentName>,

        #[arg(help = "Program to run in environment")]
        program: String,

        #[arg(help = "Zero or more arguments to pass to program")]
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    #[command(name = "generate-repositories-yaml", about = "(Experimental)")]
    GenerateRepositoriesYaml {
        #[arg(help = "Root directory for local repository", value_parser = parse_absolute_path)]
        local_repository_dir: PathBuf,
    },

    #[command(
        name = "info",
        about = "Execute command in shell for current Python environment"
    )]
    Info,

    #[command(name = "init", about = "Initialize current Python environment")]
    Init,

    #[command(
        name = "list",
        about = "List named and project Python environments and uses"
    )]
    List,

    #[command(name = "new", about = "New project Python environment")]
    New {
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },

    #[command(name = "scratch", about = "(Experimental)")]
    Scratch,

    #[command(name = "shell", about = "Start shell for current Python environment")]
    Shell {
        #[arg(help = "Environment name", short = 'e', long = "env", value_parser = parse_environment_name)]
        environment_name: Option<EnvironmentName>,
    },

    #[command(
        name = "use",
        about = "Use specified named Python environment for current directory"
    )]
    Use {
        #[arg(help = "Environment name", value_parser = parse_environment_name)]
        environment_name: EnvironmentName,
    },

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
    Version::parse(s).ok_or(String::from("invalid version"))
}

fn parse_tag(s: &str) -> Result<Tag, String> {
    Ok(Tag::parse(s))
}
