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
use super::cell::CellPtr;
use super::row::Row;
use colored::{Color, Colorize};

macro_rules! row {
    ($table: expr, $c0: expr) => {{
        $table.add_row(&[$c0.into()]);
    }};

    ($table: expr, $c0: expr, $c1: expr) => {{
        $table.add_row(&[$c0.into(), $c1.into()]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr) => {{
        $table.add_row(&[$c0.into(), $c1.into(), $c2.into()]);
    }};

    ($table: expr, $c0: expr, $c1: expr, $c2: expr, $c3: expr) => {{
        $table.add_row(&[$c0.into(), $c1.into(), $c2.into(), $c3.into()]);
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

    pub fn add_row(&mut self, cells: &[CellPtr]) {
        if self.widths.len() < cells.len() {
            self.widths.resize(cells.len(), 0);
        }

        let mut strs = Vec::with_capacity(cells.len());
        for (idx, cell) in cells.iter().enumerate() {
            let s = cell.render();
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
