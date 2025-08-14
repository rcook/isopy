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
use crate::status::{StatusResult, success, user_error};
use log::info;
use std::collections::HashSet;

pub(crate) fn do_set_config(app: &App, name: &str, value: Option<&String>) -> StatusResult {
    let names = CONFIG_NAMES.into_iter().collect::<HashSet<_>>();
    if !names.contains(name) {
        let mut names = names
            .into_iter()
            .map(|n| format!("\"{n}\""))
            .collect::<Vec<_>>();
        names.sort_unstable();
        let s = names.join(", ");
        user_error!("No such configuration value \"{name}\"; available values: {s}")
    }

    let old_value = app.get_config_value(name)?;
    if let Some(s) = value {
        app.set_config_value(name, s)?;
        match old_value {
            Some(value) => {
                info!("Configuration value \"{name}\" changed from \"{value}\" to \"{s}\"");
            }
            None => info!("Configuration value \"{name}\" set to \"{s}\""),
        }
    } else {
        app.delete_config_value(name)?;
        match old_value {
            Some(value) => {
                info!("Configuration value \"{name}\" cleared (previous value: \"{value}\")");
            }
            None => info!("Configuration value \"{name}\" cleared"),
        }
    }
    success!()
}
