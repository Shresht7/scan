use std::io::Write;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEventKind, MouseEventKind},
    style::{style, Print, Stylize},
    QueueableCommand,
};

use crate::helpers;

/// Represents a viewport
#[derive(Default, Clone, PartialEq, Eq)]
pub struct View {
    /// The string to search for in the view
    pub search: String,

    /// The index of the first-line to display in the viewport
    pub scroll_row: usize,
    /// The index of the first-column to display in the viewport
    pub scroll_col: usize,

    /// Should show line numbers
    pub show_line_numbers: bool,
    /// Should show borders
    pub show_borders: bool,

    /// The x-position (column number)
    pub x: u16,
    /// The y-position (row number)
    pub y: u16,
    /// The height of the viewport in number of rows
    pub height: usize,
    /// The width of the viewport in number of columns
    pub width: usize,

    /// The borders around the viewport
    borders: helpers::Borders,
}

impl View {
    /// The start of the viewport. Index of the first visible line
    pub fn start(&self) -> usize {
        self.scroll_row
    }

    /// The end of the viewport. Index of the last visible line
    pub fn end(&self) -> usize {
        let borders = if self.show_borders {
            self.borders.height_reduction()
        } else {
            0
        };
        self.scroll_row + self.height - borders
    }

    /// Perform setup. The setup function is called once on initialization
    pub fn setup(
        &mut self,
        stdout: &mut std::io::Stdout,
        size: (usize, usize),
    ) -> std::io::Result<()> {
        self.width = size.0;
        self.height = size.1;
        self.render_borders(stdout)?;
        Ok(())
    }

    /// Render the view component
    pub fn render(
        &self,
        stdout: &mut std::io::Stdout,
        lines: &Vec<String>,
    ) -> std::io::Result<Self> {
        // Iterate over the lines in the viewport ...
        let start = self.start();
        let end = std::cmp::min(self.end(), lines.len());
        for (i, l) in lines[start..end].iter().enumerate() {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            let mut found_something = false;

            // If the line matches the search criteria
            if !self.search.is_empty() {
                let mut highlighted_line = String::new();
                let mut remaining = &line[..];

                while let Some(start_idx) = remaining.find(&self.search) {
                    found_something = true;

                    // Add text before the match
                    highlighted_line.push_str(&remaining[..start_idx]);

                    // Add the highlighted match
                    let end_idx = start_idx + self.search.len();
                    let match_str = &remaining[start_idx..end_idx];
                    highlighted_line
                        .push_str(&style(match_str).black().on_white().bold().to_string());

                    // Move the remaining slice to after the match
                    remaining = &remaining[end_idx..];
                }

                // Add any remaining text after the last match
                highlighted_line.push_str(remaining);
                line = highlighted_line;
            }

            // Clip the string for horizontal scroll
            if self.scroll_col > 0 {
                line = match l.split_at_checked(self.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            if !self.search.is_empty() && !found_something {
                line = style(line).dark_grey().to_string();
            }

            // Prepend line numbers if the option was set
            if self.show_line_numbers {
                let line_number = format!("{:>3}", self.start() + i + 1);
                let line_number = style(line_number).dark_grey();
                let divider = style("│").dark_grey();
                line = format!("{line_number} {divider} {line}");
            }

            // Truncate the line to fit in the page width
            line = helpers::truncate_visible(
                &mut line,
                self.width
                    .saturating_sub(self.borders.width_reduction() + 2),
            );

            // Write empty whitespace to the remaining cells to clear previous buffer
            let remaining = " ".repeat(self.width.saturating_sub(
                helpers::visible_width(&line) + self.borders.width_reduction() + 2,
            ));
            line = format!("{line}{remaining}");

            // Print out the formatted line
            let x_offset = if self.show_borders { 1 } else { 0 };
            let y_offset = if self.show_borders { 1 } else { 0 };
            stdout
                .queue(cursor::MoveTo(
                    helpers::visible_width(&self.borders.left) as u16 + x_offset,
                    i as u16 + y_offset,
                ))?
                .queue(Print(line))?
                .flush()?;
        }

        Ok(self.clone())
    }

    pub fn render_borders(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        // Print top border
        if self.show_borders {
            stdout
                .queue(cursor::MoveTo(self.x, self.y))?
                .queue(Print(self.borders.top(self.width)))?;
        }

        // Apply side borders
        if self.show_borders {
            let width = self.width as u16;
            for _ in 0..self.height - 2 {
                stdout
                    .queue(Print(&style(&self.borders.left).dark_grey()))?
                    .queue(cursor::MoveToColumn(width - 1))?
                    .queue(Print(style(&self.borders.right).dark_grey()))?
                    .queue(cursor::MoveToNextLine(1))?;
            }
        }

        // Print bottom border
        if self.show_borders {
            stdout.queue(Print(self.borders.bottom(self.width)))?;
        }

        Ok(())
    }

    pub fn handle_events(
        &mut self,
        event: &Event,
        lines: &Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.scroll_up(1),
                    KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1, lines),
                    KeyCode::Left | KeyCode::Char('h') => self.scroll_left(1),
                    KeyCode::Right | KeyCode::Char('l') => self.scroll_right(1),
                    KeyCode::PageUp => self.page_up(),
                    KeyCode::PageDown => self.page_down(lines),
                    KeyCode::Home => self.home(),
                    _ => false,
                }
            }
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => self.scroll_up(1),
                MouseEventKind::ScrollDown => self.scroll_down(1, lines),
                _ => false,
            },
            _ => false,
        })
    }

    /// Scroll up by the given number of lines
    fn scroll_up(&mut self, n: usize) -> bool {
        if self.start() > 0 {
            self.scroll_row = self.scroll_row.saturating_sub(n);
        }
        return false;
    }

    /// Scroll down by the given number of lines
    fn scroll_down(&mut self, n: usize, lines: &Vec<String>) -> bool {
        if self.end() < lines.len() {
            self.scroll_row = self.scroll_row.saturating_add(n);
        }
        return false;
    }

    /// Scroll left horizontally by the given number of columns
    fn scroll_left(&mut self, n: usize) -> bool {
        self.scroll_col = self.scroll_col.saturating_sub(n);
        return false;
    }

    /// Scroll right horizontally by the given number of columns
    fn scroll_right(&mut self, n: usize) -> bool {
        self.scroll_col = self.scroll_col.saturating_add(n);
        return false;
    }

    /// Scroll up by one page
    fn page_up(&mut self) -> bool {
        if self.start() > self.height {
            self.scroll_row = self.scroll_row.saturating_sub(self.height - 1)
        } else {
            self.scroll_row = 0;
        }
        return false;
    }

    // Scroll down by one page
    fn page_down(&mut self, lines: &Vec<String>) -> bool {
        if self.end() + self.height < lines.len() {
            self.scroll_row = self.scroll_row.saturating_add(self.height - 1)
        } else if self.end() < lines.len() {
            self.scroll_row = lines.len() - self.height + 1;
        }
        return false;
    }

    /// Scroll to the home position.
    /// If there is no horizontal scroll, scrolls directly to the top of the file.
    /// Otherwise, scroll back to the start of the line first.
    /// Next invocation will bring it back to the top if there was no horizontal scroll.
    fn home(&mut self) -> bool {
        if self.scroll_col > 0 {
            self.scroll_col = 0
        } else {
            self.scroll_row = 0;
        }
        return false;
    }
}
