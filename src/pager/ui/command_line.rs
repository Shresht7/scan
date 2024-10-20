use std::io::Write;

use crossterm::{
    cursor::{self, MoveToColumn},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{style, Print, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::helpers::visible_width;

/// Represents the Command Line component of the Pager application.
/// This is where the user can input their search queries and goto commands.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct CommandLine {
    /// Stores the user input
    pub input: String,

    /// The current [mode][Mode] of the command-line
    pub mode: Mode,

    /// The x-position (column number)
    pub x: u16,
    /// The y-position (row number)
    pub y: u16,
    /// The height in number of rows
    pub height: usize,
    /// The width in number of columns
    pub width: usize,
}

/// Describes the states the command-line can be in
#[derive(Default, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Base,
    Goto,
    Search,
}

impl CommandLine {
    // SETUP
    // -----

    /// Performs the setup to initialize or re-initialize the component
    pub fn setup(&mut self, pos: (u16, u16), size: (usize, usize)) -> std::io::Result<()> {
        self.x = pos.0;
        self.y = pos.1;
        self.width = size.0;
        self.height = size.1;
        Ok(())
    }

    // RENDER
    // ------

    /// The render function is responsible for rendering the component out to the screen
    pub fn render(&self, stdout: &mut std::io::Stdout) -> std::io::Result<Self> {
        stdout
            .queue(cursor::MoveTo(self.x, self.y))?
            .queue(Clear(ClearType::CurrentLine))?;
        self.render_help(stdout)?;
        self.render_mode(stdout)?;
        self.render_input(stdout)?;
        stdout.flush()?;
        Ok(self.clone()) // Return a clone of this frame so that we can cache it and determine if we need to re-render
    }

    /// Shows the current mode of the operations of the command-line
    fn render_mode(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let mode = match self.mode {
            Mode::Base => style(""),
            Mode::Goto => style(" GOTO ").black().on_cyan(),
            Mode::Search => style(" SEARCH ").black().on_dark_yellow(),
        };
        stdout.queue(Print(mode))?;
        Ok(())
    }

    /// Renders the user-input on the command-line for visual feedback
    fn render_input(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        stdout.queue(Print(" "))?; // Apply some padding

        let cursor = style("|").rapid_blink();

        if self.input.len() > 0 {
            stdout.queue(Print(&self.input))?.queue(Print(cursor))?;
        } else {
            let placeholder = style(match self.mode {
                Mode::Search => "Enter Search Query...",
                Mode::Goto => "Enter Line or Line:Column",
                Mode::Base => "",
            })
            .dark_grey()
            .italic();
            if self.mode != Mode::Base {
                stdout.queue(Print(cursor))?.queue(Print(placeholder))?;
            }
        }
        Ok(())
    }

    /// Renders the contextual help message
    fn render_help(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let enter = style("Enter").dark_green();
        let esc = style("Esc").dark_green();
        let slash = style("/").dark_green();
        let colon = style(":").dark_green();
        let comma = style(", ").dark_grey().italic();
        let ctrl_f = style("Ctrl+F").dark_green();
        let ctrl_g = style("Ctrl+G").dark_green();
        let search = style("Find").dark_grey().italic();
        let goto = style("Goto").dark_grey().italic();
        let submit = style("Submit").dark_grey().italic();
        let back = style("Back").dark_grey().italic();
        let quit = style("Quit").dark_grey().italic();
        let dot = style("•").dark_grey();
        let help_message = match self.mode {
            Mode::Search => {
                format!("{enter} {submit} {dot} {ctrl_g} {goto} {dot} {esc} {back}")
            }
            Mode::Goto => {
                format!("{enter} {submit} {dot} {ctrl_f} {search} {dot} {esc} {back}")
            }
            Mode::Base => {
                format!(
                    "{ctrl_f}{comma}{slash} {search} {dot} {ctrl_g}{comma}{colon} {goto} {dot} {esc} {quit}"
                )
            }
        };
        stdout
            .queue(cursor::MoveToColumn(
                self.width
                    .saturating_sub(visible_width(&help_message.to_string()) + 1)
                    as u16,
            ))?
            .queue(Print(help_message))?
            .queue(MoveToColumn(self.x))?;
        Ok(())
    }

    // EVENT HANDLERS
    // --------------

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
