use std::io::Write;

use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    QueueableCommand,
};

use super::View;
use crate::helpers;

impl View {
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

            let mut found_something = false;

            // If the line matches the search criteria
            if !self.search.is_empty() {
                let mut highlighted_line = String::new();
                let mut remaining = &line[..];

                while let Some(start_idx) = remaining.find(&self.search) {
                    found_something = true;

                    // Add text before the match
                    highlighted_line.push_str(&remaining[..start_idx]);

                    // Add the highlighted match
                    let end_idx = start_idx + self.search.len();
                    let match_str = &remaining[start_idx..end_idx];
                    highlighted_line
                        .push_str(&style(match_str).black().on_white().bold().to_string());

                    // Move the remaining slice to after the match
                    remaining = &remaining[end_idx..];
                }

                // Add any remaining text after the last match
                highlighted_line.push_str(remaining);
                line = highlighted_line;
            }

            // Clip the string for horizontal scroll
            if self.scroll_col > 0 {
                line = match l.split_at_checked(self.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            if !self.search.is_empty() && !found_something {
                line = style(line).dark_grey().to_string();
            }

            // Prepend line numbers if the option was set
            if self.show_line_numbers {
                let line_number = format!("{:>3}", self.start() + i + 1);
                let line_number = style(line_number).dark_grey();
                let divider = style("│").dark_grey();
                line = format!("{line_number} {divider} {line}");
            }

            // Truncate the line to fit in the page width
            line = helpers::truncate_visible(
                &mut line,
                self.width
                    .saturating_sub(self.borders.width_reduction() + 2),
            );

            // Write empty whitespace to the remaining cells to clear previous buffer
            let remaining = " ".repeat(self.width.saturating_sub(
                helpers::visible_width(&line) + self.borders.width_reduction() + 2,
            ));
            line = format!("{line}{remaining}");

            // Print out the formatted line
            let x_offset = if self.show_borders { 1 } else { 0 };
            let y_offset = if self.show_borders { 1 } else { 0 };
            stdout
                .queue(cursor::MoveTo(
                    helpers::visible_width(&self.borders.left) as u16 + x_offset,
                    i as u16 + y_offset,
                ))?
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
