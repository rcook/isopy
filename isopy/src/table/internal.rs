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
use colored::{Color, Colorize};
use std::fmt::Display;

macro_rules! row {
    ($table: expr, $c0: expr) => {{
        $table.add_row(&[Box::new($c0)]);
    }};

    ($table: expr, $c0: expr, $c1: expr) => {{
        $table.add_row(&[Box::new($c0), Box::new($c1)]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr) => {{
        $table.add_row(&[Box::new($c0), Box::new($c1), Box::new($c2)]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr, $c3: expr) => {{
        $table.add_row(&[Box::new($c0), Box::new($c1), Box::new($c2), Box::new($c3)]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr, $c3: expr, $c4: expr) => {{
        $table.add_row(&[
            Box::new($c0),
            Box::new($c1),
            Box::new($c2),
            Box::new($c3),
            Box::new($c4),
        ]);
    }};
}

pub(crate) use row;

pub struct Table {
    divider_indent: usize,
    columns_indent: usize,
    cell_padding: usize,
    divider_colour: Color,
    default_cell_colour: Color,
    cell_colours: Vec<Color>,
    widths: Vec<usize>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(
        divider_indent: usize,
        columns_indent: usize,
        cell_padding: usize,
        divider_colour: Color,
        default_cell_colour: Color,
        cell_colours: &[Color],
    ) -> Self {
        Self {
            divider_indent,
            columns_indent,
            cell_padding,
            divider_colour,
            default_cell_colour,
            cell_colours: cell_colours.to_vec(),
            widths: vec![],
            rows: vec![],
        }
    }

    pub fn add_divider(&mut self, s: &str) {
        self.rows.push(Row::Divider(String::from(s)));
    }

    pub fn add_row(&mut self, values: &[Box<dyn Display>]) {
        let values_len = values.len();
        if self.widths.len() < values_len {
            self.widths.resize(values_len, 0);
        }

        let mut strs = Vec::with_capacity(values.len());
        for (idx, value) in values.iter().enumerate() {
            let s = value.to_string();
            self.widths[idx] = self.widths[idx].max(s.len());
            strs.push(s);
        }

        self.rows.push(Row::Columns(strs));
    }

    pub fn print(&self) {
        for row in &self.rows {
            match row {
                Row::Divider(s) => println!(
                    "{:<width$}{}",
                    "",
                    s.color(self.divider_colour),
                    width = self.divider_indent
                ),
                Row::Columns(strs) => {
                    print!("{:<width$}", "", width = self.columns_indent);
                    for (idx, s) in strs.iter().enumerate() {
                        let colour = self
                            .cell_colours
                            .get(idx)
                            .unwrap_or(&self.default_cell_colour);
                        print!("{:<width$}", s.color(*colour), width = self.widths[idx]);
                        print!("{:<width$}", "", width = self.cell_padding);
                    }
                    println!();
                }
            }
        }
    }
}
