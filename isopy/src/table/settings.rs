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
use super::internal::Table;
use colored::Color;

pub struct TableSettings {
    pub divider_indent: usize,
    pub columns_indent: usize,
    pub cell_padding: usize,
    pub divider_colour: Color,
    pub default_cell_colour: Color,
    pub cell_colours: Vec<Color>,
}

impl TableSettings {
    pub fn build(&self) -> Table {
        Table::new(
            self.divider_indent,
            self.columns_indent,
            self.cell_padding,
            self.divider_colour,
            self.default_cell_colour,
            &self.cell_colours,
        )
    }
}

impl Default for TableSettings {
    fn default() -> Self {
        Self {
            divider_indent: 0,
            columns_indent: 0,
            cell_padding: 2,
            divider_colour: Color::BrightYellow,
            default_cell_colour: Color::BrightWhite,
            cell_colours: vec![],
        }
    }
}
