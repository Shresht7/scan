use super::Pager;

impl Pager {
    /// Render the Pager's view
    pub fn render(
        &mut self,
        stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render the view component
        if self.view != self.last_frame {
            self.last_frame = self.view.render(stdout, &self.lines)?;
        }

        Ok(())
    }
}
