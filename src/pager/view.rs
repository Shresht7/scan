use std::io::Write;

use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    QueueableCommand,
};

use crate::helpers;

/// Represents a viewport
#[derive(Default, Clone, PartialEq, Eq)]
pub struct View {
    /// The index of the first-line to display in the viewport
    pub scroll_row: usize,
    /// The index of the first-column to display in the viewport
    pub scroll_col: usize,

    /// Should show line numbers
    pub show_line_numbers: bool,
    /// Show borders
    pub show_borders: bool,

    /// The x-position of the element
    pub x: u16,
    /// The y-position of the element
    pub y: u16,

    /// The height of the viewport
    pub height: usize,
    /// The width of the viewport
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
        let borders = if self.show_borders { 2 } else { 0 };
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

    /// Render the view component
    pub fn render(
        &self,
        stdout: &mut std::io::Stdout,
        lines: &Vec<String>,
    ) -> std::io::Result<Self> {
        // Iterate over the lines in the viewport ...
        let start = self.start();
        let end = std::cmp::min(self.end(), lines.len());
        for (i, l) in lines[start..end].iter().enumerate() {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            // Clip the string for horizontal scroll
            if self.scroll_col > 0 {
                line = match l.split_at_checked(self.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            // Prepend line numbers if the option was set
            if self.show_line_numbers {
                let line_number = format!("{:>3}", self.start() + i + 1);
                let line_number = style(line_number).dark_grey();
                let divider = style("│").dark_grey();
                line = format!("{line_number} {divider} {line}");
            }

            // Truncate the line to fit in the page width
            line = helpers::truncate_visible(&mut line, self.width);

            // Write empty whitespace to the remaining cells to clear previous buffer
            let remaining = " ".repeat(self.width - helpers::visible_width(&line));
            line = format!("{line}{remaining}");

            // Print out the formatted line
            stdout
                .queue(cursor::MoveTo(2, i as u16 + 1))?
                .queue(Print(line))?
                .flush()?;
        }

        Ok(self.clone())
    }

    pub fn render_borders(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        // Print top border
        if self.show_borders {
            stdout
                .queue(cursor::MoveTo(self.x, self.y))?
                .queue(Print(self.borders.top(self.width)))?;
        }

        // Apply side borders
        if self.show_borders {
            let width = self.width as u16;
            for _ in 0..self.height - 2 {
                stdout
                    .queue(Print(&style(&self.borders.left).dark_grey()))?
                    .queue(cursor::MoveToColumn(width - 1))?
                    .queue(Print(style(&self.borders.right).dark_grey()))?
                    .queue(cursor::MoveToNextLine(1))?;
            }
        }

        // Print bottom border
        if self.show_borders {
            stdout.queue(Print(self.borders.bottom(self.width)))?;
        }

        Ok(())
    }
}
