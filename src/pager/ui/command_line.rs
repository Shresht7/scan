use std::io::Write;

use crossterm::{cursor, style::Print, QueueableCommand};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct CommandLine {
    /// The x-position of the element
    pub x: u16,
    /// The y-position of the element
    pub y: u16,

    /// The height of the element
    pub height: usize,
    /// The width of the element
    pub width: usize,
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
        stdout
            .queue(cursor::MoveTo(self.x, self.y))?
            .queue(Print("Command Line"))?
            .flush()?;

        Ok(self.clone())
    }
}
