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
use super::row::Row;
use super::settings::TableSettings;
use colored::Colorize;

macro_rules! table_title {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_title(&std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_title;

macro_rules! table_divider {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_divider(&std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_divider;

macro_rules! table_row {
    ($table: expr, $c0: expr) => {{
        $table.add_row(&[&format!("{}", $c0)]);
    }};

    ($table: expr, $c0: expr, $c1: expr) => {{
        $table.add_row(&[&format!("{}", $c0), &format!("{}", $c1)]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr) => {{
        $table.add_row(&[
            &format!("{}", $c0),
            &format!("{}", $c1),
            &format!("{}", $c2),
        ]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr, $c3: expr) => {{
        $table.add_row(&[
            &format!("{}", $c0),
            &format!("{}", $c1),
            &format!("{}", $c2),
            &format!("{}", $c3),
        ]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr, $c3: expr, $c4: expr) => {{
        $table.add_row(&[
            &format!("{}", $c0),
            &format!("{}", $c1),
            &format!("{}", $c2),
            &format!("{}", $c3),
            &format!("{}", $c4),
        ]);
    }};
}
pub(crate) use table_row;

macro_rules! table_line {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_line(&std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_line;

pub struct Table {
    settings: TableSettings,
    widths: Vec<usize>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(settings: &TableSettings) -> Self {
        Self {
            settings: settings.clone(),
            widths: vec![],
            rows: vec![],
        }
    }

    pub fn add_title(&mut self, s: &str) {
        self.rows.push(Row::Title(String::from(s)));
    }

    pub fn add_divider(&mut self, s: &str) {
        self.rows.push(Row::Divider(String::from(s)));
    }

    pub fn add_row(&mut self, values: &[&str]) {
        let values_len = values.len();
        if self.widths.len() < values_len {
            self.widths.resize(values_len, 0);
        }

        let mut strs = Vec::with_capacity(values.len());
        for (idx, value) in values.iter().enumerate() {
            let s = (*value).to_string();
            self.widths[idx] = self.widths[idx].max(s.len());
            strs.push(s);
        }

        self.rows.push(Row::Columns(strs));
    }

    pub fn add_line(&mut self, s: &str) {
        self.rows.push(Row::Line(String::from(s)));
    }

    pub fn print(&self) {
        for row in &self.rows {
            match row {
                Row::Title(s) => println!(
                    "{:<width$}{}",
                    "",
                    s.color(self.settings.title_colour),
                    width = self.settings.title_indent
                ),
                Row::Divider(s) => println!(
                    "{:<width$}{}",
                    "",
                    s.color(self.settings.divider_colour),
                    width = self.settings.divider_indent
                ),
                Row::Columns(strs) => {
                    if !strs.is_empty() {
                        print!("{:<width$}", "", width = self.settings.columns_indent);

                        for (idx, s) in strs.iter().enumerate() {
                            if idx > 0 {
                                print!("{}", self.settings.column_separator);
                            }

                            let colour = self
                                .settings
                                .column_colours
                                .get(idx)
                                .unwrap_or(&self.settings.default_column_colour);
                            print!("{:<width$}", s.color(*colour), width = self.widths[idx]);
                        }
                        println!();
                    }
                }
                Row::Line(s) => println!(
                    "{:<width$}{}",
                    "",
                    s.color(self.settings.line_colour),
                    width = self.settings.line_indent
                ),
            }
        }
    }
}
