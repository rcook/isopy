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
mod available;
mod check;
mod download;
mod downloaded;
mod exec;
mod gen_config;
mod info;
mod init;
mod init_config;
mod link;
mod list;
mod prompt;
mod scratch;
mod shell;
mod wrap;

pub use self::available::do_available;
pub use self::check::do_check;
pub use self::download::do_download;
pub use self::downloaded::do_downloaded;
pub use self::exec::do_exec;
pub use self::gen_config::do_gen_config;
pub use self::info::do_info;
pub use self::init::do_init;
pub use self::init_config::do_init_config;
pub use self::link::do_link;
pub use self::list::do_list;
pub use self::prompt::do_prompt;
pub use self::scratch::do_scratch;
pub use self::shell::do_shell;
pub use self::wrap::do_wrap;
