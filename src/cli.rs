use clap::Parser;

use crate::helpers::File;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// The file to view. If nothing is specified, use input from STDIN
    pub file: Option<File>,

    /// Show line numbers (aliases: --line-numbers, --numbers)
    #[clap(short = 'l', long, aliases=["line-numbers", "numbers"])]
    pub show_line_numbers: bool,

    /// Show borders around the contents
    #[clap(short = 'b', long)]
    pub show_borders: bool,

    /// Pass the contents through without running the interactive Pager
    #[clap(short, long, aliases=["skip", "no-page"])]
    pub passthrough: bool,

    /// Read the entire file in one go
    #[clap(short, long)]
    pub all: bool,
}
