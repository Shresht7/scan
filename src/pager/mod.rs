use crossterm::{
    cursor,
    style::{style, Stylize},
    terminal, ExecutableCommand,
};

mod events;
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
            read_all: false,
            rerender: true,
            exit: false,
        }
    }

    /// Enable/Disable line numbers
    pub fn with_line_numbers(&mut self, b: bool) -> &mut Self {
        self.show_line_numbers = b;
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

    /// Render the Pager's view
    fn render(&mut self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        // Skip rendering if self.rerender is set to false
        if !self.rerender {
            return Ok(());
        }

        // Clear the screen and move the cursor to the top
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // Iterate over the lines in the viewport ...
        let start = self.view.start();
        let end = std::cmp::min(self.view.end(), self.lines.len());
        for (i, l) in self.lines[start..end].iter().enumerate() {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            // Clip the string for horizontal scroll
            if self.view.scroll_col > 0 {
                line = match l.split_at_checked(self.view.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            // Prepend line numbers if the option was set
            if self.show_line_numbers {
                let line_number = format!("{:>3}", self.view.start() + i + 1);
                let line_number = style(line_number).dark_grey();
                let divider = style("│").dark_grey();
                line = format!("{line_number} {divider} {line}");
            }

            // Truncate the line to fit in the page width
            line.truncate(self.view.width);

            // Print out the formatted line
            println!("{line}");
        }

        // Reset the rerender flag after rendering
        self.last_frame = self.view.clone();
        self.rerender = false;

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

    /// Determines if we need to rerender the view
    fn should_rerender(&mut self) {
        // If the current frame is different from the last...
        if self.view != self.last_frame {
            return self.rerender = true; // Rerender
        }
        return self.rerender = false;
    }

    /// Set the exit flag to indicate that we need to exit the program
    fn exit(&mut self) {
        self.exit = true;
    }
}
