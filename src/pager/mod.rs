use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    terminal, QueueableCommand,
};

mod events;
mod render;
mod view;

pub struct Pager {
    /// The collection of buffered lines
    lines: Vec<String>,

    /// The current Pager's view
    view: view::View,

    /// Stores a snapshot of the previously rendered view.
    last_frame: view::View,

    /// Should show line numbers
    show_line_numbers: bool,

    /// Show borders
    show_borders: bool,

    // Should read the entire file in one go
    read_all: bool,

    /// Should rerender the view
    rerender: bool,

    /// If true, exit the program
    exit: bool,
}

impl Pager {
    /// Instantiate the Pager application
    pub fn init(size: (u16, u16)) -> Pager {
        Self {
            lines: Vec::new(),
            view: view::View::new(0, 0, size),
            last_frame: view::View::new(0, 0, size),
            show_line_numbers: false,
            show_borders: false,
            read_all: false,
            rerender: false,
            exit: false,
        }
    }

    /// Enable/Disable line numbers
    pub fn with_line_numbers(&mut self, b: bool) -> &mut Self {
        self.show_line_numbers = b;
        self
    }

    /// Enable/Disable borders
    pub fn with_borders(&mut self, yes: bool) -> &mut Self {
        self.show_borders = yes;
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
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        // Instantiate the borders
        let borders = render::Borders::default();

        // Print top border
        if self.show_borders {
            stdout
                .queue(cursor::MoveTo(0, 0))?
                .queue(Print(borders.top(self.view.width - 2)))?;
        }

        // Apply side borders
        if self.show_borders {
            let width = self.view.width as u16;
            for _ in 0..self.view.height - 2 {
                stdout
                    .queue(Print(&style(&borders.left).dark_grey()))?
                    .queue(cursor::MoveToColumn(width - 1))?
                    .queue(Print(style(&borders.right).dark_grey()))?
                    .queue(cursor::MoveToNextLine(1))?;
            }
        }

        // Print bottom border
        if self.show_borders {
            let height = self.view.height as u16;
            stdout
                .queue(cursor::MoveTo(0, height - 2))?
                .queue(Print(borders.bottom(self.view.width - 2) + "\n"))?;
        }

        // Buffer initial set of lines
        self.buffer_lines(&mut reader)?;

        // The main program loop. Break when the exit flag is set.
        while !self.exit {
            // Buffer more lines as needed based on the self.scroll and self.page_height variables
            self.buffer_lines(&mut reader)?;

            // Render the pager's view
            self.render(&mut stdout)?;

            // Handle key events before continuing to loop
            self.handle_events(&mut reader)?;

            // Determine if we need to render the view
            self.should_rerender();
        }

        Ok(())
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
