use crossterm::{
    cursor,
    style::{style, Stylize},
    terminal, ExecutableCommand,
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

        // Clear the screen and move the cursor to the top
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // Instantiate the borders
        let borders = Borders::default();

        // Print top border
        if self.show_borders {
            println!("{}", borders.top(self.view.width + 2));
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
            line.truncate(self.view.width);

            // Apply side borders
            if self.show_borders {
                line = borders.wrap(&line, self.view.width);
            }

            // Print out the formatted line
            println!("{line}");
        }

        // Print bottom border
        if self.show_borders {
            println!("{}", borders.bottom(self.view.width + 2));
        }

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

struct Borders {
    top: String,
    bottom: String,
    left: String,
    right: String,
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
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
    fn top(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            self.top_left,
            self.top.repeat(width),
            self.top_right
        )
    }

    /// Wrap a text line with side borders
    fn wrap(&self, line: &str, width: usize) -> String {
        if line.visible_width() < width {
            let remaining = " ".repeat(width - line.visible_width());
            return format!("{} {line}{remaining} {}", self.left, self.right);
        } else {
            return line.to_string();
        }
    }

    /// Draw the bottom border
    fn bottom(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            self.bottom_left,
            self.bottom.repeat(width),
            self.bottom_right
        )
    }
}
