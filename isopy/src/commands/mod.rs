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
mod check;
mod completions;
mod config_values;
mod default;
mod download;
mod env;
mod info;
mod init;
mod link;
mod list;
mod packages;
mod project;
mod prompt;
mod remove;
mod run;
mod scratch;
mod shell;
mod tags;
mod update;
mod wrap;

pub(crate) use check::do_check;
pub(crate) use completions::do_completions;
pub(crate) use config_values::do_config_values;
pub(crate) use default::do_default;
pub(crate) use download::do_download;
pub(crate) use env::do_env;
pub(crate) use info::do_info;
pub(crate) use init::do_init;
pub(crate) use link::do_link;
pub(crate) use list::do_list;
pub(crate) use packages::do_packages;
pub(crate) use project::do_project;
pub(crate) use prompt::do_prompt;
pub(crate) use remove::do_remove;
pub(crate) use run::do_run;
pub(crate) use scratch::do_scratch;
pub(crate) use shell::do_shell;
pub(crate) use tags::do_tags;
pub(crate) use update::do_update;
pub(crate) use wrap::do_wrap;
