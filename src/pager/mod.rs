use crossterm::{cursor, terminal, ExecutableCommand};

mod events;
mod view;

pub struct Pager {
    /// The collection of buffered lines
    lines: Vec<String>,

    /// The current Pager's view
    view: view::View,

    /// Stores a snapshot of the previously rendered view.
    last_frame: view::View,

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
            rerender: true,
            exit: false,
        }
    }

    /// The main application logic of the pager
    pub fn run<T>(&mut self, mut reader: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Buffer initial set of lines
        self.buffer_lines(&mut reader)?;

        // Prepare stdout by entering the Alternate Screen Buffer,
        // clearing the terminal and moving the cursor to the 0, 0 position
        let mut stdout = std::io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // The main program loop. Break when the exit flag is set.
        while !self.exit {
            // Buffer more lines as needed based on the self.scroll and self.page_height variables
            self.buffer_lines(&mut reader)?;

            // Render the pager's view
            self.render(&mut stdout)?;

            // Handle key events before continuing to loop
            self.handle_events()?;

            // Determine if we need to render the view
            self.should_rerender();
        }

        // Restore the terminal by exiting the Alternate Screen Buffer when we're done
        stdout.execute(terminal::LeaveAlternateScreen)?;

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
        for l in &self.lines[self.view.start()..self.view.end()] {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            // Clip the string for horizontal scroll
            if self.view.scroll_col > 0 {
                line = match l.split_at_checked(self.view.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            // Truncate the line to fit in the page width
            line.truncate(self.view.width);

            // Print out the formatted line
            println!("{}", line);
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
            // Read up to the viewport's end + one more page
            if self.lines.len() > self.view.end() + self.view.height {
                break;
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
