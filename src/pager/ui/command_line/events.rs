use super::{CommandLine, Mode};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

impl CommandLine {
    /// Event handler for the command-line component. Returns true to stop to prevent event propagation
    pub fn handle_events(&mut self, event: &Event) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(match self.mode {
            Mode::Search | Mode::Goto => self.handle_search_and_goto_mode_events(event)?,
            Mode::Base => self.handle_base_mode_events(event)?,
        })
    }

    /// Handles events when in [Search][Mode::Search] or [Goto][Mode::Goto] mode
    fn handle_search_and_goto_mode_events(
        &mut self,
        event: &Event,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    self.mode = Mode::Base;
                    self.input.clear();
                    return Ok(true);
                }
                KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('f'),
                    ..
                } => {
                    self.mode = Mode::Search;
                    self.input.clear();
                    return Ok(true);
                }
                KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('g'),
                    ..
                } => {
                    self.mode = Mode::Goto;
                    self.input.clear();
                    return Ok(true);
                }
                KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                } => {
                    if self.mode == Mode::Goto {
                        if c == &':' || c.is_numeric() {
                            self.input.push(c.clone());
                        }
                        return Ok(true);
                    }
                    self.input.push(c.clone());
                    return Ok(true);
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        let mut words: Vec<&str> = self.input.split(" ").collect();
                        words.pop();
                        self.input = words.join(" ");
                    } else {
                        self.input.pop();
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(false)
    }

    /// Handles events when in [Base mode][Mode::Base]
    fn handle_base_mode_events(
        &mut self,
        event: &Event,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event {
                // Switch to Search Mode
                KeyEvent {
                    code: KeyCode::Char('/'),
                    ..
                }
                | KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('f'),
                    ..
                } => self.mode = Mode::Search,

                // Switch to Goto Mode
                KeyEvent {
                    code: KeyCode::Char(':') | KeyCode::Char(';'),
                    ..
                }
                | KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('g'),
                    ..
                } => self.mode = Mode::Goto,

                // Catch all
                _ => {}
            },
            _ => {}
        }
        Ok(false)
    }
}
