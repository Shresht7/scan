use super::View;

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};

impl View {
    pub fn handle_events(
        &mut self,
        event: &Event,
        lines: &Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.scroll_up(1),
                    KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1, lines),
                    KeyCode::Left | KeyCode::Char('h') => self.scroll_left(1),
                    KeyCode::Right | KeyCode::Char('l') => self.scroll_right(1),
                    KeyCode::PageUp => self.page_up(),
                    KeyCode::PageDown => self.page_down(lines),
                    KeyCode::Home => self.home(),
                    _ => false,
                }
            }
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => self.scroll_up(1),
                MouseEventKind::ScrollDown => self.scroll_down(1, lines),
                _ => false,
            },
            _ => false,
        })
    }

    /// Scroll up by the given number of lines
    fn scroll_up(&mut self, n: usize) -> bool {
        if self.start() > 0 {
            self.scroll_row = self.scroll_row.saturating_sub(n);
        }
        return false;
    }

    /// Scroll down by the given number of lines
    fn scroll_down(&mut self, n: usize, lines: &Vec<String>) -> bool {
        if self.end() < lines.len() {
            self.scroll_row = self.scroll_row.saturating_add(n);
        }
        return false;
    }

    /// Scroll left horizontally by the given number of columns
    fn scroll_left(&mut self, n: usize) -> bool {
        self.scroll_col = self.scroll_col.saturating_sub(n);
        return false;
    }

    /// Scroll right horizontally by the given number of columns
    fn scroll_right(&mut self, n: usize) -> bool {
        self.scroll_col = self.scroll_col.saturating_add(n);
        return false;
    }

    /// Scroll up by one page
    fn page_up(&mut self) -> bool {
        if self.start() > self.height {
            self.scroll_row = self.scroll_row.saturating_sub(self.height - 1)
        } else {
            self.scroll_row = 0;
        }
        return false;
    }

    // Scroll down by one page
    fn page_down(&mut self, lines: &Vec<String>) -> bool {
        if self.end() + self.height < lines.len() {
            self.scroll_row = self.scroll_row.saturating_add(self.height - 1)
        } else if self.end() < lines.len() {
            self.scroll_row = lines.len() - self.height + 1;
        }
        return false;
    }

    /// Scroll to the home position.
    /// If there is no horizontal scroll, scrolls directly to the top of the file.
    /// Otherwise, scroll back to the start of the line first.
    /// Next invocation will bring it back to the top if there was no horizontal scroll.
    fn home(&mut self) -> bool {
        if self.scroll_col > 0 {
            self.scroll_col = 0
        } else {
            self.scroll_row = 0;
        }
        return false;
    }

    // NOTE: Scrolling to the end is handled at the Pager level because the view component doesn't
    // NOTE: know the extent of the total number of lines yet as they haven't been buffered.
    // NOTE: Once the Pager read all the contents, it can signal down the scroll_col value to go to the end.
}
