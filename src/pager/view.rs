use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    QueueableCommand,
};

use crate::helpers::ANSIString;

/// Represents a viewport
#[derive(Clone, PartialEq, Eq)]
pub struct View {
    /// The index of the first-line to display in the viewport
    pub scroll_row: usize,
    /// The index of the first-column to display in the viewport
    pub scroll_col: usize,

    /// Should show line numbers
    pub show_line_numbers: bool,

    // The height of the viewport
    pub height: usize,
    // The width of the viewport
    pub width: usize,
}

impl View {
    /// Instantiates a new View instance
    pub fn new(scroll_row: usize, scroll_col: usize, width: usize, height: usize) -> Self {
        Self {
            scroll_row,
            scroll_col,
            show_line_numbers: false,
            height,
            width,
        }
    }

    /// The start of the viewport. Index of the first visible line
    pub fn start(&self) -> usize {
        self.scroll_row
    }

    /// The end of the viewport. Index of the last visible line
    pub fn end(&self) -> usize {
        self.scroll_row + self.height
    }

    /// Render the view component
    pub fn render(&self, stdout: &mut std::io::Stdout, lines: &Vec<String>) -> std::io::Result<()> {
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
            line.as_str().truncate_visible(self.width);

            // Write empty whitespace to the remaining cells to clear previous buffer
            let remaining = " ".repeat(self.width - 5 - line.as_str().visible_width());
            line = format!("{line}{remaining}");

            // Print out the formatted line
            stdout
                .queue(cursor::MoveTo(2, i as u16 + 1))?
                .queue(Print(line))?;
        }
        Ok(())
    }
}
