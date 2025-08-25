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
use anyhow::{Result, anyhow};
use clap::Parser;
use clap::Subcommand as ClapSubcommand;
use joat_repo::{MetaId, SharedPath};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        global = true,
        long = "dir",
        short = 'd',
        help = "Path to repository directory"
    )]
    pub repo_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    #[command(name = "find", about = "Find parent metadirectory")]
    Find,

    #[command(name = "info", about = "Show configuration")]
    Info,

    #[command(name = "init", about = "Initialize metadirectory")]
    Init,

    #[command(name = "ln", about = "Create link to existing metadirectory")]
    Link {
        #[arg(
            long = "ref",
            short = 'r',
            help = "Existing metadirectory ID",
            value_parser = parse_meta_id
        )]
        meta_id: Option<MetaId>,
    },

    #[command(name = "ls", about = "Show all metadirectory info")]
    List,

    #[command(name = "purge", about = "Purge repository")]
    Purge {
        #[arg(
            long = "force",
            default_value = "false",
            help = "Really purge repository"
        )]
        force: bool,
    },

    #[command(name = "read", about = "Read string from shared file")]
    Read {
        #[arg(name = "path", help = "Path", value_parser = parse_shared_path)]
        path: SharedPath,
    },

    #[command(name = "rm", about = "Unlink metadirectory")]
    Remove,

    #[command(name = "show", about = "Show metadirectory info")]
    Show,

    #[command(name = "trash", about = "Show trash and optionally clean up")]
    Trash {
        #[arg(long = "clean", default_value = "false", help = "Clean up")]
        clean: bool,
    },

    #[command(name = "write", about = "Save string to shared file")]
    Write {
        #[arg(name = "path", help = "Path", value_parser = parse_shared_path)]
        path: SharedPath,

        #[arg(name = "value", help = "String")]
        value: String,
    },
}

fn parse_meta_id(s: &str) -> Result<MetaId> {
    s.parse::<MetaId>().map_err(|e| anyhow!(e))
}

#[allow(clippy::unnecessary_wraps)]
fn parse_shared_path(s: &str) -> Result<SharedPath> {
    Ok(SharedPath::new(s))
}
