use crate::object_model::EnvName;
use crate::object_model::{Tag, Version};
use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result;

#[derive(Parser, Debug)]
#[command(
    about = "Isolated Python Tool Rust Edition!",
    after_help = "https://rcook.github.io/isopy/\nhttps://github.com/rcook/isopyrs\nhttps://github.com/rcook/isopy",
    version
)]
pub struct Args {
    #[arg(help = "Path to isopy cache directory", short = 'd', long = "dir", value_parser = parse_absolute_path)]
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
        #[arg(help = "Environment name", value_parser = parse_env_name)]
        env_name: EnvName,
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
        #[arg(help = "Environment name", short = 'e', long = "env", value_parser = parse_env_name)]
        env_name: Option<EnvName>,

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
        about = "List named and anonymous Python environments and uses"
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
    Scratch {
        #[arg(help = "Output path", value_parser = parse_absolute_path)]
        index_json_path1: PathBuf,

        #[arg(help = "Output path", value_parser = parse_absolute_path)]
        index_json_path2: PathBuf,
    },

    #[command(name = "shell", about = "Start shell for current Python environment")]
    Shell {
        #[arg(help = "Environment name", short = 'e', long = "env", value_parser = parse_env_name)]
        env_name: Option<EnvName>,
    },

    #[command(
        name = "use",
        about = "Use specified named Python environment for current directory"
    )]
    Use {
        #[arg(help = "Environment name", value_parser = parse_env_name)]
        env_name: EnvName,
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

fn parse_env_name(s: &str) -> Result<EnvName, String> {
    EnvName::parse(s).ok_or(String::from("invalid env name"))
}

fn parse_version(s: &str) -> Result<Version, String> {
    Version::parse(s).ok_or(String::from("invalid version"))
}

fn parse_tag(s: &str) -> Result<Tag, String> {
    Ok(Tag::parse(s))
}
