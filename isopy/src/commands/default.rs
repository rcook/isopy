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
use crate::constants::DEFAULT_MONIKER_CONFIG_NAME;
use crate::moniker::Moniker;
use crate::status::{success, StatusResult};

pub(crate) fn do_default(app: &App, moniker: &Option<Moniker>) -> StatusResult {
    let old_value = app.get_config_value(DEFAULT_MONIKER_CONFIG_NAME)?;

    if let Some(m) = moniker {
        app.set_config_value(DEFAULT_MONIKER_CONFIG_NAME, m.as_str())?;
        match old_value {
            Some(value) => println!(
                "Default package manager changed from {value} to {new_value}",
                new_value = m.as_str()
            ),
            None => println!(
                "Default package manager changed to {new_value}",
                new_value = m.as_str()
            ),
        }
    } else {
        app.delete_config_value(DEFAULT_MONIKER_CONFIG_NAME)?;
        match old_value {
            Some(value) => println!("Default package manager cleared (previous value: {value})"),
            None => println!("Default package manager cleared"),
        }
    };
    success!()
}
