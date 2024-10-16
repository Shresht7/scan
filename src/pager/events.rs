use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};

use super::Pager;

impl Pager {
    /// Handle crossterm events like key-presses
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        match crossterm::event::read()? {
            // It's important to check that the event is a key-press event as
            // crossterm also emits key-release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.scroll_up(1),
                    KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1),
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => self.scroll_up(1),
                MouseEventKind::ScrollDown => self.scroll_down(1),
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    /// Scroll up by the given number of lines
    fn scroll_up(&mut self, n: usize) {
        if self.view_start() > 0 {
            self.scroll_offset = self.scroll_offset.saturating_sub(n);
        }
    }

    /// Scroll down by the given number of lines
    fn scroll_down(&mut self, n: usize) {
        if self.view_end() < self.lines.len() {
            self.scroll_offset = self.scroll_offset.saturating_add(n);
        }
    }
}
