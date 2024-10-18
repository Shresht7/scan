use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};

use super::Pager;

impl Pager {
    /// Handle crossterm events like key-presses
    pub fn handle_events<T>(&mut self, reader: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        match crossterm::event::read()? {
            // It's important to check that the event is a key-press event as
            // crossterm also emits key-release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.scroll_up(1),
                    KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1),
                    KeyCode::Left | KeyCode::Char('h') => self.scroll_left(1),
                    KeyCode::Right | KeyCode::Char('l') => self.scroll_right(1),
                    KeyCode::PageUp => self.page_up(),
                    KeyCode::PageDown => self.page_down(),
                    KeyCode::Home => self.home(),
                    KeyCode::End => self.end(reader)?,
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => self.scroll_up(1),
                MouseEventKind::ScrollDown => self.scroll_down(1),
                _ => {}
            },
            Event::Resize(w, h) => self.resize(w, h),
            _ => {}
        }
        Ok(())
    }

    /// Scroll up by the given number of lines
    fn scroll_up(&mut self, n: usize) {
        if self.view.start() > 0 {
            self.view.scroll_row = self.view.scroll_row.saturating_sub(n);
        }
    }

    /// Scroll down by the given number of lines
    fn scroll_down(&mut self, n: usize) {
        if self.view.end() < self.lines.len() {
            self.view.scroll_row = self.view.scroll_row.saturating_add(n);
        }
    }

    /// Scroll left horizontally by the given number of columns
    fn scroll_left(&mut self, n: usize) {
        self.view.scroll_col = self.view.scroll_col.saturating_sub(n);
    }

    /// Scroll right horizontally by the given number of columns
    fn scroll_right(&mut self, n: usize) {
        self.view.scroll_col = self.view.scroll_col.saturating_add(n);
    }

    /// Scroll up by one page
    fn page_up(&mut self) {
        if self.view.start() > self.view.height {
            self.view.scroll_row = self.view.scroll_row.saturating_sub(self.view.height - 1)
        } else {
            self.view.scroll_row = 0;
        }
    }

    // Scroll down by one page
    fn page_down(&mut self) {
        if self.view.end() + self.view.height < self.lines.len() {
            self.view.scroll_row = self.view.scroll_row.saturating_add(self.view.height - 1)
        } else if self.view.end() < self.lines.len() {
            self.view.scroll_row = self.lines.len() - self.view.height + 1;
        }
    }

    /// Scroll to the home position.
    /// If there is no horizontal scroll, scrolls directly to the top of the file.
    /// Otherwise, scroll back to the start of the line first.
    /// Next invocation will bring it back to the top if there was no horizontal scroll.
    fn home(&mut self) {
        if self.view.scroll_col > 0 {
            self.view.scroll_col = 0
        } else {
            self.view.scroll_row = 0;
        }
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
    fn resize(&mut self, w: u16, h: u16) {
        let border_size = if self.show_borders { 2 } else { 0 };
        self.view.width = w as usize - border_size - 2;
        self.view.height = h as usize - border_size;
    }
}
