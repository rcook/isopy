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
use crate::object_model::{OpenJdkVersion, Tag, Version};
use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use joat_repo::MetaId;
use log::LevelFilter;
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

    #[arg(
        global = true,
        help = "Logging level",
        short = 'l',
        long = "level",
        default_value_t = LogLevel::Info,
        value_enum
    )]
    pub log_level: LogLevel,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(
        name = "available",
        about = "List Python packages available for download"
    )]
    Available {
        #[arg(
            help = "Package filter",
            short = 'p',
            long = "package",
            default_value_t = PackageFilter::Python,
            value_enum
        )]
        package_filter: PackageFilter,
    },

    #[command(
        name = "check",
        about = "Check integrity of repository and optionally clean up"
    )]
    Check {
        #[arg(long = "clean", default_value = "false", help = "Clean up")]
        clean: bool,
    },

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
    GenConfig {
        #[clap(flatten)]
        python_version: PythonVersion,

        #[arg(
            short = 'f',
            long = "force",
            help = "Force overwrite of .python-version.yaml file"
        )]
        force: bool,
    },

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

    #[command(name = "prompt", about = "Show brief information in prompt")]
    Prompt,

    #[command(name = "scratch", about = "Experimental stuff")]
    Scratch {
        #[arg(help = "OpenJDK version")]
        openjdk_version: OpenJdkVersion,
    },

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

#[derive(Clone, Debug, ValueEnum)]
pub enum LogLevel {
    #[clap(name = "off")]
    Off,

    #[clap(name = "error")]
    Error,

    #[clap(name = "warn")]
    Warn,

    #[clap(name = "info")]
    Info,

    #[clap(name = "debug")]
    Debug,

    #[clap(name = "trace")]
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum PackageFilter {
    #[clap(name = "all")]
    All,

    #[clap(name = "python")]
    Python,

    #[clap(name = "openjdk")]
    OpenJdk,
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}

fn parse_version(s: &str) -> Result<Version, String> {
    s.parse::<Version>()
        .map_err(|_| String::from("invalid version"))
}

fn parse_tag(s: &str) -> Result<Tag, String> {
    s.parse::<Tag>().map_err(|_| String::from("invalid tag"))
}

fn parse_meta_id(s: &str) -> Result<MetaId, String> {
    s.parse::<MetaId>()
        .map_err(|_| String::from("invalid meta ID"))
}
