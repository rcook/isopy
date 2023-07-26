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
use crate::app::App;
use crate::print::{make_prop_table, print_dir_info_and_env, print_repo};
use crate::status::{return_success, Status};
use crate::table::{table_columns, table_title};
use anyhow::Result;

pub fn info(app: &App) -> Result<Status> {
    let mut table = make_prop_table();

    table_title!(table, "Current directory");

    table_columns!(table, "Working directory", app.cwd().display());

    if let Some(dir_info) = app.find_dir_info(None)? {
        print_dir_info_and_env(&mut table, &dir_info)?;
    }

    table_title!(table, "Repository information");
    print_repo(&mut table, app.repo());

    table.print();
    return_success!();
}
