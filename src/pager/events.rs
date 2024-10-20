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
        // If the event handler returns a true, then the event propagation must stop now and we exit early
        if self.command_line.handle_events(&event)? {
            return Ok(());
        }
        if self.view.handle_events(&event, &self.lines)? {
            return Ok(());
        }

        match event {
            // It's important to check that the event is a key-press event as
            // crossterm also emits key-release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::End => self.end(reader)?,
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    KeyCode::Enter => match self.command_line.mode {
                        Mode::Search => self.view.search = self.command_line.input.clone(),
                        Mode::Goto => {
                            let input = self.command_line.input.clone();
                            self.command_line.input.clear();
                            let (row, col) = helpers::parse_row_and_col(&input);
                            self.view.scroll_row = row.unwrap_or(1).saturating_sub(1);
                            self.view.scroll_col = col.unwrap_or(1).saturating_sub(1);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            Event::Resize(w, h) => self.resize(w, h, stdout)?,
            _ => {}
        }

        Ok(())
    }

    /// Scroll to the end position.
    /// Reads the entire file to the buffer.
    fn end<T>(&mut self, reader: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        self.read_all = true;
        self.buffer_lines(reader)?;
        self.view.scroll_row = self.lines.len() - self.view.height + 1;
        Ok(())
    }

    /// Resize the Pager view
    pub fn resize(&mut self, w: u16, h: u16, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        self.width = w as usize;
        self.height = h as usize;
        self.setup(stdout)?;
        Ok(())
    }
}
