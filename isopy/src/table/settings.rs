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

#[derive(Clone, Debug)]
pub(crate) struct TableSettings {
    pub(crate) title_indent: usize,
    pub(crate) divider_indent: usize,
    pub(crate) columns_indent: usize,
    pub(crate) line_indent: usize,
    pub(crate) title_colour: Color,
    pub(crate) divider_colour: Color,
    pub(crate) default_column_colour: Color,
    pub(crate) column_colours: Vec<Color>,
    pub(crate) default_heading_colour: Color,
    pub(crate) heading_colours: Vec<Color>,
    pub(crate) line_colour: Color,
    pub(crate) column_separator: String,
}

impl TableSettings {
    pub(crate) fn build(&self) -> Table {
        Table::new(self)
    }
}

impl Default for TableSettings {
    fn default() -> Self {
        Self {
            title_indent: 0,
            divider_indent: 0,
            columns_indent: 0,
            line_indent: 0,
            title_colour: Color::Cyan,
            divider_colour: Color::BrightYellow,
            default_column_colour: Color::BrightWhite,
            column_colours: vec![],
            default_heading_colour: Color::BrightGreen,
            heading_colours: vec![],
            line_colour: Color::BrightWhite,
            column_separator: String::from("  "),
        }
    }
}
