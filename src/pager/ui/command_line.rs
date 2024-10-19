use std::io::Write;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{style, Print, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct CommandLine {
    /// Stores the user input
    pub input: String,

    /// The current mode of the command-line
    pub mode: Mode,

    /// The x-position of the element
    pub x: u16,
    /// The y-position of the element
    pub y: u16,

    /// The height of the element
    pub height: usize,
    /// The width of the element
    pub width: usize,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Base,
    Goto,
    Search,
}

impl CommandLine {
    pub fn setup(&mut self, pos: (u16, u16), size: (usize, usize)) -> std::io::Result<()> {
        self.x = pos.0;
        self.y = pos.1;
        self.width = size.0;
        self.height = size.1;
        Ok(())
    }

    pub fn render(&self, stdout: &mut std::io::Stdout) -> std::io::Result<Self> {
        let mode = match self.mode {
            Mode::Base => style(""),
            Mode::Goto => style(" GOTO ").black().on_cyan(),
            Mode::Search => style(" SEARCH ").black().on_dark_yellow(),
        };

        stdout
            .queue(cursor::MoveTo(self.x, self.y))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(mode))?;

        if self.input.len() > 0 {
            stdout.queue(Print(&self.input))?;
        }

        stdout.flush()?;

        Ok(self.clone())
    }

    pub fn handle_events(&mut self, event: &Event) -> Result<bool, Box<dyn std::error::Error>> {
        let mut stop_event_propagation = false;
        match self.mode {
            Mode::Search | Mode::Goto => match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Base;
                            self.input.clear();
                            stop_event_propagation = true;
                        }
                        KeyCode::Char(c) => {
                            if self.mode == Mode::Goto && !c.is_numeric() {
                                return Ok(true);
                            }
                            self.input.push(c)
                        }
                        KeyCode::Backspace => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                let mut words: Vec<&str> = self.input.split(" ").collect();
                                words.pop();
                                self.input = words.join(" ");
                            } else {
                                self.input.pop();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            Mode::Base => match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('/') => self.mode = Mode::Search,
                        KeyCode::Char(':') | KeyCode::Char(';') => self.mode = Mode::Goto,
                        KeyCode::Esc => self.mode = Mode::Base,
                        _ => {}
                    }
                }
                _ => {}
            },
        }
        Ok(stop_event_propagation)
    }
}
