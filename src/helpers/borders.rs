use crossterm::style::{style, Stylize};

pub struct Borders {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            top: "─".into(),
            bottom: "─".into(),
            left: "│".into(),
            right: "│".into(),
            top_left: "┌".into(),
            top_right: "┐".into(),
            bottom_left: "└".into(),
            bottom_right: "┘".into(),
        }
    }
}

impl Borders {
    /// Draw the top border
    pub fn top(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            style(&self.top_left).dark_grey(),
            style(&self.top.repeat(width - 2)).dark_grey(),
            style(&self.top_right).dark_grey()
        )
    }

    /// Draw the bottom border
    pub fn bottom(&self, width: usize) -> String {
        format!(
            "{}{}{}",
            style(&self.bottom_left).dark_grey(),
            style(&self.bottom.repeat(width - 2)).dark_grey(),
            style(&self.bottom_right).dark_grey()
        )
    }
}
