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
use crate::table::row::Row;
use crate::table::settings::TableSettings;
use colored::{Color, Colorize};

macro_rules! table_title {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_title(&::std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_title;

macro_rules! table_divider {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_divider(&::std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_divider;

macro_rules! table_columns {
    ($table: expr) => {
        $table.add_columns(&[""; 0])
    };

    ($table: expr, $head: expr $(, $tail: expr) *) => {
        $table.add_columns(&[&$head.to_string() $(, &$tail.to_string())*])
    };
}
pub(crate) use table_columns;

macro_rules! table_headings {
    ($table: expr) => {
        $table.add_headings(&[""; 0])
    };

    ($table: expr, $head: expr $(, $tail: expr) *) => {
        $table.add_headings(&[&$head.to_string() $(, &$tail.to_string())*])
    };
}
pub(crate) use table_headings;

macro_rules! table_line {
    ($table: expr, $($arg: tt)*) => {{
        $table.add_line(&::std::fmt::format(format_args!($($arg)*)));
    }}
}
pub(crate) use table_line;

pub(crate) struct Table {
    settings: TableSettings,
    widths: Vec<usize>,
    rows: Vec<Row>,
}

impl Table {
    pub(crate) fn new(settings: &TableSettings) -> Self {
        Self {
            settings: settings.clone(),
            widths: vec![],
            rows: vec![],
        }
    }

    pub(crate) fn add_title(&mut self, s: &str) {
        self.rows.push(Row::Title(String::from(s)));
    }

    pub(crate) fn add_divider(&mut self, s: &str) {
        self.rows.push(Row::Divider(String::from(s)));
    }

    pub(crate) fn add_columns(&mut self, values: &[&str]) {
        let columns = self.make_column_vec(values);
        self.rows.push(Row::Columns(columns));
    }

    pub(crate) fn add_headings(&mut self, values: &[&str]) {
        let columns = self.make_column_vec(values);
        self.rows.push(Row::Headings(columns));
    }

    pub(crate) fn add_line(&mut self, s: &str) {
        self.rows.push(Row::Line(String::from(s)));
    }

    pub(crate) fn print(&self) {
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
                Row::Columns(strs) => self.print_columns(
                    strs,
                    &self.settings.column_colours,
                    self.settings.default_column_colour,
                ),
                Row::Headings(strs) => self.print_columns(
                    strs,
                    &self.settings.heading_colours,
                    self.settings.default_heading_colour,
                ),
                Row::Line(s) => println!(
                    "{:<width$}{}",
                    "",
                    s.color(self.settings.line_colour),
                    width = self.settings.line_indent
                ),
            }
        }
    }

    fn make_column_vec(&mut self, values: &[&str]) -> Vec<String> {
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

        strs
    }

    fn print_columns(&self, strs: &[String], colours: &[Color], default_colour: Color) {
        if !strs.is_empty() {
            print!("{:<width$}", "", width = self.settings.columns_indent);

            for (idx, s) in strs.iter().enumerate() {
                if idx > 0 {
                    print!("{}", self.settings.column_separator);
                }

                let colour = colours.get(idx).unwrap_or(&default_colour);
                print!("{:<width$}", s.color(*colour), width = self.widths[idx]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::table::internal::{Table, table_columns};
    use crate::table::settings::TableSettings;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    #[test]
    fn table_columns_macro() {
        struct MyStruct;

        impl Display for MyStruct {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "EMPTY")
            }
        }

        let settings = TableSettings::default();
        let mut table = Table::new(&settings);
        assert_eq!(0, table.rows.len());
        table_columns!(table);
        assert_eq!(1, table.rows.len());
        table_columns!(table, 1);
        assert_eq!(2, table.rows.len());
        table_columns!(table, 1, "hello");
        assert_eq!(3, table.rows.len());
        table_columns!(table, 1, "hello", MyStruct {});
        assert_eq!(4, table.rows.len());
    }
}
