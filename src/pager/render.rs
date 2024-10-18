use std::io::Write;

use crossterm::style::{style, Stylize};

use super::Pager;

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

        // Render the view component
        self.view.render(stdout, &self.lines)?;

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
