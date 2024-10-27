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
        &mut self,
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

                for (ln, si, ei) in &self.search_matches {
                    if start + i == *ln as usize {
                        found_something = true;

                        // Add text before the match
                        highlighted_line.push_str(&line[..*si as usize]);

                        // Get the currently selected match
                        let selected = self.search_matches
                            [self.search_match_index % self.search_matches.len()];

                        // If selected is out of view, scroll to it
                        if selected.0 > self.end() as u16 {
                            self.scroll_row = selected.0 as usize;
                        }

                        // Add the highlighted match
                        let match_str = &line[*si as usize..*ei as usize];
                        let format_match_str =
                            if selected.0 == *ln && selected.1 == *si && selected.2 == *ei {
                                style(match_str).black().on_yellow().bold().to_string()
                            } else {
                                style(match_str).black().on_white().bold().to_string()
                            };
                        highlighted_line.push_str(&format_match_str);

                        // Add any remaining text after the last match
                        highlighted_line.push_str(&line[*ei as usize..]);
                    }
                }

                if found_something {
                    line = highlighted_line;
                }
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
