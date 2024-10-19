use super::Pager;

impl Pager {
    /// Render the Pager's view
    pub fn render(
        &mut self,
        stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render the view component
        if self.view != self.prev.view {
            self.prev.view = self.view.render(stdout, &self.lines)?;
        }
        // Render the command line component
        if self.command_line != self.prev.command_line {
            self.prev.command_line = self.command_line.render(stdout)?;
        }
        Ok(())
    }
}
