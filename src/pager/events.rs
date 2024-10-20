use crossterm::event::{Event, KeyCode, KeyEventKind};

use super::{ui::Mode, Pager};
use crate::helpers;

impl Pager {
    /// Handle crossterm events like key-presses, mouse-scroll and window resize
    pub fn handle_events<T>(
        &mut self,
        reader: T,
        stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Read crossterm event
        let event = crossterm::event::read()?;

        // Call sub-component event-handlers
        // If the event handlers returns a true, then the event propagation must stop now and we exit early
        if self.command_line.handle_events(&event)? {
            return Ok(());
        }
        if self.view.handle_events(&event, &self.lines)? {
            return Ok(());
        }

        // Call global event-handler
        self.handle_global_events(event, reader, stdout)?;

        Ok(())
    }

    /// Handle global level events
    fn handle_global_events<T>(
        &mut self,
        event: Event,
        reader: T,
        stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        Ok(match event {
            // It's important to check that the event is a key-press event as
            // crossterm also emits key-release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::End => self.go_to_end(reader)?,
                    KeyCode::Enter => self.handle_command_line_submit(),
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            Event::Resize(w, h) => self.resize(w, h, stdout)?,
            _ => {}
        })
    }

    /// Command-line submit event handlers
    fn handle_command_line_submit(&mut self) {
        match self.command_line.mode {
            Mode::Search => self.search(),
            Mode::Goto => self.goto(),
            _ => {}
        }
    }

    /// Search for the given input
    fn search(&mut self) {
        self.view.search = self.command_line.input.clone()
    }

    /// Jump to the provided line number and column
    fn goto(&mut self) {
        let input = self.command_line.input.clone();
        self.command_line.input.clear();
        let (row, col) = helpers::parse_row_and_col(&input);
        self.view.scroll_row = row.unwrap_or(1).saturating_sub(1);
        self.view.scroll_col = col.unwrap_or(1).saturating_sub(1);
    }

    /// Read and scroll to the end position.
    /// Reads the entire file to the buffer.
    fn go_to_end<T>(&mut self, reader: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        self.read_all = true; // Set the flag to read all contents from the reader
        self.buffer_lines(reader)?; // Read the contents
        self.view.scroll_row = self.lines.len() - self.view.height + 1; // Update the scroll view position
        Ok(())
    }

    /// Resize event handler
    pub fn resize(&mut self, w: u16, h: u16, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        self.width = w as usize;
        self.height = h as usize;
        self.setup(stdout)?;
        Ok(())
    }
}
