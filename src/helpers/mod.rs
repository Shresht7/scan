mod ansi;
mod borders;
mod file;
pub mod layout;

pub use ansi::*;
pub use borders::*;
pub use file::*;

use crossterm::{
    style::{style, Stylize},
    tty::IsTty,
};

/// Prints the human friendly error message
pub fn print_error(e: Box<dyn std::error::Error>) {
    let message = style(format!("Error: {e}")).red();
    if std::io::stderr().is_tty() {
        eprintln!("{message}")
    }
}
