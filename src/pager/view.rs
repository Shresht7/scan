/// Represents a viewport
#[derive(Clone)]
pub struct View {
    /// The index of the first-line to display in the viewport
    pub scroll_row: usize,
    /// The index of the first-column to display in the viewport
    pub scroll_col: usize,
    /// The max height of the page in the terminal
    pub height: usize,
    /// The max width of the page in the terminal
    pub width: usize,
}

impl View {
    /// Instantiates a new View instance
    pub fn new(scroll_row: usize, scroll_col: usize, size: (u16, u16)) -> Self {
        Self {
            scroll_row,
            scroll_col,
            height: size.1 as usize,
            width: size.0 as usize,
        }
    }
}
