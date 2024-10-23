use crate::helpers;

mod events;
mod render;

/// Represents a viewport
#[derive(Default, Clone, PartialEq, Eq)]
pub struct View {
    /// The string to search for in the view
    pub search: String,

    /// The index of the first-line to display in the viewport
    pub scroll_row: usize,
    /// The index of the first-column to display in the viewport
    pub scroll_col: usize,

    /// Should show line numbers
    pub show_line_numbers: bool,
    /// Should show borders
    pub show_borders: bool,

    /// The x-position (column number)
    pub x: u16,
    /// The y-position (row number)
    pub y: u16,
    /// The height of the viewport in number of rows
    pub height: usize,
    /// The width of the viewport in number of columns
    pub width: usize,

    /// The borders around the viewport
    borders: helpers::Borders,
}

impl View {
    /// The start of the viewport. Index of the first visible line
    pub fn start(&self) -> usize {
        self.scroll_row
    }

    /// The end of the viewport. Index of the last visible line
    pub fn end(&self) -> usize {
        let borders = if self.show_borders {
            self.borders.height_reduction()
        } else {
            0
        };
        self.scroll_row + self.height - borders
    }

    /// Perform setup. The setup function is called once on initialization
    pub fn setup(
        &mut self,
        stdout: &mut std::io::Stdout,
        size: (usize, usize),
    ) -> std::io::Result<()> {
        self.width = size.0;
        self.height = size.1;
        self.render_borders(stdout)?;
        Ok(())
    }
}
