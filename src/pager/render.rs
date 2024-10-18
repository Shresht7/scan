use std::io::Write;

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
