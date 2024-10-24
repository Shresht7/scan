use std::io::Write;

use crossterm::{
    cursor,
    style::{style, Print, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use super::{CommandLine, Mode};
use crate::helpers;

impl CommandLine {
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
            Mode::Search => style(" FIND ").black().on_dark_yellow(),
        };
        stdout.queue(Print(" "))?.queue(Print(mode))?;
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
        let find = style("Find").dark_grey().italic();
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
                format!("{enter} {submit} {dot} {ctrl_f} {find} {dot} {esc} {back}")
            }
            Mode::Base => {
                format!(
                        "{ctrl_f}{comma}{slash} {find} {dot} {ctrl_g}{comma}{colon} {goto} {dot} {esc} {quit}"
                    )
            }
        };
        stdout
            .queue(cursor::MoveToColumn(
                self.width
                    .saturating_sub(helpers::visible_width(&help_message.to_string()) + 1)
                    as u16,
            ))?
            .queue(Print(help_message))?
            .queue(cursor::MoveToColumn(self.x))?;
        Ok(())
    }
}
