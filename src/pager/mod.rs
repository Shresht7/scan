use crate::helpers::layout;

mod events;
mod render;
mod ui;

#[derive(Default)]
pub struct Pager {
    /// The collection of buffered lines
    lines: Vec<String>,

    /// The current Pager's view
    view: ui::View,

    /// The application's command line
    command_line: ui::CommandLine,

    /// The currently focussed ui
    focus: Focus,

    /// Stores a snapshot of the previously rendered view.
    prev: PreviousFrame,

    // Should read the entire file in one go
    read_all: bool,

    /// Width of the application
    width: usize,
    /// Height of the application
    height: usize,

    /// If true, exit the program
    exit: bool,
}

#[derive(Default)]
pub enum Focus {
    View,
    #[default]
    CommandLine,
}

#[derive(Default)]
struct PreviousFrame {
    view: ui::View,
    command_line: ui::CommandLine,
}

impl Pager {
    /// Instantiate the Pager application
    pub fn init(size: (u16, u16)) -> Pager {
        Self {
            width: size.0 as usize,
            height: size.1 as usize,
            ..Default::default()
        }
    }

    /// Enable/Disable line numbers
    pub fn with_line_numbers(&mut self, yes: bool) -> &mut Self {
        self.view.show_line_numbers = yes;
        self
    }

    /// Enable/Disable borders
    pub fn with_borders(&mut self, yes: bool) -> &mut Self {
        self.view.show_borders = yes;
        self
    }

    /// Set the starting scroll offsets
    pub fn with_offset(&mut self, row: Option<usize>, col: Option<usize>) -> &mut Self {
        self.view.scroll_row = row.unwrap_or(0).saturating_sub(1);
        self.view.scroll_col = col.unwrap_or(0).saturating_sub(1);
        self
    }

    /// Set the read_all option
    pub fn all(&mut self, yes: bool) -> &mut Self {
        self.read_all = yes;
        self
    }

    /// The main application logic of the pager
    pub fn run<T>(
        &mut self,
        mut reader: T,
        mut stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Perform setup
        self.setup(&mut stdout)?;

        // The main program loop. Break when the exit flag is set.
        while !self.exit {
            // Buffer lines as needed; based on the viewport
            self.buffer_lines(&mut reader)?;

            // Render the pager's view
            self.render(&mut stdout)?;

            // Handle key events before continuing to loop
            self.handle_events(&mut reader, stdout)?;
        }

        Ok(())
    }

    /// Perform setup. The setup function is run once at the start.
    fn setup(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        // Determine the layout sizes
        let sizes = layout::distribute(
            self.height,
            &vec![layout::Size::Flexible, layout::Size::Fixed(1)],
        );

        // Setup subcomponents
        self.view.setup(stdout, (self.width, sizes[0]))?;
        self.command_line
            .setup((0, self.height as u16), (self.width, sizes[1]))?;

        Ok(())
    }

    /// Set the focus on an element
    pub fn set_focus(&mut self, to: Focus) {
        self.focus = to;
        match self.focus {
            Focus::View => {
                self.command_line.is_focussed = false;
            }
            Focus::CommandLine => {
                self.command_line.is_focussed = true;
            }
        }
    }

    // HELPER FUNCTIONS
    // ----------------

    /// Buffer lines from the reader as needed
    fn buffer_lines<T>(&mut self, reader: T) -> std::io::Result<()>
    where
        T: std::io::BufRead,
    {
        for line in reader.lines() {
            self.lines.push(line?);
            // Read only up to the viewport's end + one more page unless the self.read_all flag is set
            if !self.read_all {
                if self.lines.len() > self.view.end() + self.view.height {
                    break;
                }
            }
        }
        Ok(())
    }

    /// Set the exit flag to indicate that we need to exit the program
    fn exit(&mut self) {
        self.exit = true;
    }
}
