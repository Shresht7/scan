use std::str::FromStr;

#[derive(Clone)]
pub struct File {
    filename: String,
    pub row: Option<usize>,
    pub col: Option<usize>,
}

impl FromStr for File {
    type Err = String; // Define the error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        let filename = iter.next().unwrap_or(s).to_string();
        let row = iter.next().and_then(|s| s.parse::<usize>().ok());
        let col = iter.next().and_then(|s| s.parse::<usize>().ok());
        Ok(Self { filename, row, col })
    }
}

/// Returns a BufReader. If a filepath is specified, returns a BufReader for the File,
/// otherwise, returns a BufReader for STDIN.
pub fn get_reader(
    file: &Option<File>,
) -> Result<Box<dyn std::io::BufRead>, Box<dyn std::error::Error>> {
    let reader: Box<dyn std::io::BufRead> = if let Some(file) = &file {
        let filepath = std::path::Path::new(&file.filename);
        if !filepath.exists() {
            return Err(format!("The provided file does not exist: {}", file.filename).into());
        }
        let file = std::fs::File::open(filepath)?;
        Box::new(std::io::BufReader::new(file))
    } else {
        Box::new(std::io::stdin().lock())
    };
    Ok(reader)
}

/// Character that denotes the starts of escape codes
const ESC: char = '\x1b';

pub trait ANSIString {
    fn visible_width(&self) -> usize;
    fn truncate_visible(&mut self, width: usize) -> &mut Self;
}

impl ANSIString for &str {
    fn visible_width(&self) -> usize {
        let mut width: usize = 0;
        let mut chars = self.chars();

        while let Some(c) = chars.next() {
            // If we have not encountered a ESC yet ...
            if c != ESC {
                width += 1
            } else {
                // .. otherwise, we hit the start of an ESC sequence
                while let Some(c) = chars.next() {
                    if c == 'm' {
                        break; // Break as soon as we encounter the end of an ansi-code
                    }
                }
            }
        }

        return width;
    }

    fn truncate_visible(&mut self, width: usize) -> &mut Self {
        let visible_width = self.visible_width();
        if visible_width > width {
            self.to_string().truncate(width);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normal_width_for_regular_strings() {
        let str = "Hello World!";
        assert_eq!(str.len(), str.visible_width())
    }

    #[test]
    fn should_correctly_account_for_ansi_codes() {
        let str = "Hello World!";
        let ansi_str = "\x1b[31mHello World!\x1b[0m";
        assert_eq!(str.len(), ansi_str.visible_width())
    }
}
