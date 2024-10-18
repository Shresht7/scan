use std::io::Write;

use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    QueueableCommand,
};

use super::Pager;
use crate::helpers::ANSIString;

impl Pager {
    /// Render the Pager's view
    pub fn render(
        &mut self,
        stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Skip rendering if self.rerender is set to false
        if !self.rerender {
            return Ok(());
        }

        // Iterate over the lines in the viewport ...
        let start = self.view.start();
        let end = std::cmp::min(self.view.end(), self.lines.len());
        for (i, l) in self.lines[start..end].iter().enumerate() {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            // Clip the string for horizontal scroll
            if self.view.scroll_col > 0 {
                line = match l.split_at_checked(self.view.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            // Prepend line numbers if the option was set
            if self.show_line_numbers {
                let line_number = format!("{:>3}", self.view.start() + i + 1);
                let line_number = style(line_number).dark_grey();
                let divider = style("│").dark_grey();
                line = format!("{line_number} {divider} {line}");
            }

            // Truncate the line to fit in the page width
            line.as_str().truncate_visible(self.view.width);

            // Write empty whitespace to the remaining cells to clear previous buffer
            let remaining = " ".repeat(self.view.width - 5 - line.as_str().visible_width());
            line = format!("{line}{remaining}");

            // Print out the formatted line
            stdout
                .queue(cursor::MoveTo(2, i as u16 + 1))?
                .queue(Print(line))?;
        }

        // Queue the terminal commands and the output
        stdout.flush()?;

        // Reset the rerender flag after rendering
        self.last_frame = self.view.clone();
        self.rerender = false;

        Ok(())
    }

    // HELPER FUNCTIONS
    // ----------------

    /// Determines if we need to rerender the view
    pub fn should_rerender(&mut self) {
        // If the current frame is different from the last...
        if self.view != self.last_frame {
            return self.rerender = true; // Rerender
        }
        return self.rerender = false;
    }
}

pub struct Borders {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            top: "─".into(),
            bottom: "─".into(),
            left: "│".into(),
            right: "│".into(),
            top_left: "┌".into(),
            top_right: "┐".into(),
            bottom_left: "└".into(),
            bottom_right: "┘".into(),
        }
    }
}

impl Borders {
    /// Draw the top border
    pub fn top(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            style(&self.top_left).dark_grey(),
            style(&self.top.repeat(width)).dark_grey(),
            style(&self.top_right).dark_grey()
        )
    }

    /// Draw the bottom border
    pub fn bottom(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            style(&self.bottom_left).dark_grey(),
            style(&self.bottom.repeat(width)).dark_grey(),
            style(&self.bottom_right).dark_grey()
        )
    }
}
