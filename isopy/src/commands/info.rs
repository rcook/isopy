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
use crate::constants::CONFIG_NAMES;
use crate::env::{get_env_keys, read_env};
use crate::print::{make_prop_table, print_dir_info_and_env, print_repo};
use crate::status::{success, StatusResult};
use crate::table::{table_columns, table_title};

const NO_VALUE: &str = "(not set)";

pub(crate) fn do_info(app: &App) -> StatusResult {
    let mut table = make_prop_table();

    table_title!(table, "Current directory");

    table_columns!(table, "Working directory", app.cwd.display());

    if let Some(dir_info) = app.find_dir_info(None)? {
        print_dir_info_and_env(app, &mut table, &dir_info)?;
    }

    table_title!(table, "Repository information");
    print_repo(&mut table, &app.repo);

    table_title!(table, "Configuration");
    table_columns!(table, "Configuration directory", app.config_dir.display());
    table_columns!(
        table,
        "Configuration values file",
        app.config_value_path.display()
    );

    table_title!(table, "Environment variables");
    let mut env_keys = get_env_keys();
    env_keys.sort_by(|a, b| a.name().cmp(b.name()));
    for env_key in env_keys {
        let value = read_env(env_key)?;
        let value_str = value.as_deref().unwrap_or(NO_VALUE);
        table_columns!(table, env_key, value_str);
    }

    table_title!(table, "Configuration values");
    let config_values = app.get_config_values()?;
    let mut names = CONFIG_NAMES.into_iter().collect::<Vec<_>>();
    names.sort_unstable();
    for name in names {
        match config_values.get(name) {
            Some(value) => table_columns!(table, name, value),
            None => table_columns!(table, name, NO_VALUE),
        }
    }

    table.print();

    success!();
}
