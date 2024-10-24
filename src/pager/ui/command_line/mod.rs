mod events;
mod render;

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
    /// Performs the setup to initialize or re-initialize the component
    pub fn setup(&mut self, pos: (u16, u16), size: (usize, usize)) -> std::io::Result<()> {
        self.x = pos.0;
        self.y = pos.1;
        self.width = size.0;
        self.height = size.1;
        Ok(())
    }
}
