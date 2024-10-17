use clap::Parser;

use crate::helpers::File;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// The file to view. If nothing is specified, use input from STDIN
    pub filename: Option<File>,

    /// Show line numbers
    #[clap(short, long, aliases=["line-numbers"])]
    pub show_line_numbers: bool,

    /// Pass the contents through without running the interactive Pager
    #[clap(short, long, aliases=["skip", "no-page"])]
    pub passthrough: bool,
}
