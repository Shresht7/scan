use std::io::Write;

use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct CommandLine {
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
            .queue(Print(mode))?
            .flush()?;

        Ok(self.clone())
    }
}
