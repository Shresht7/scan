/// Character that denotes the starts of escape codes
const ESC: char = '\x1b';

/// Determine the length of characters in an ANSI string that are visible in the terminal
pub fn visible_width(s: &str) -> usize {
    let mut width: usize = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        // If we have not encountered a ESC yet ...
        if c != ESC {
            width += 1
        } else {
            // .. otherwise, we hit the start of an ESC sequence
            while let Some(c) = chars.next() {
                match c {
                    // Control Sequence Introducer: continue until `\x40-\x7C` (ASCII @A–Z[\]^_`a–z{|}~).
                    // See https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences
                    '[' => {
                        if matches!(chars.next(), Some('\x40'..='\x7C') | None) {
                            break; // Break as soon as we encounter the end of an ansi-code
                        }
                    }
                    _ => width += 1,
                }
            }
        }
    }

    width
}

pub fn truncate_visible(s: &str, width: usize) -> String {
    let mut trunc = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        trunc.push(c);
        if visible_width(&trunc) >= width {
            break;
        }
    }
    trunc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_normal_width_for_regular_strings() {
        let str = "Hello World!";
        assert_eq!(str.len(), visible_width(str))
    }

    #[test]
    fn should_return_zero_for_empty_string() {
        assert_eq!(0, visible_width(""))
    }

    #[test]
    fn should_correctly_account_for_ansi_codes() {
        let str = "Hello World!";
        let ansi_str = "\x1b[31mHello World!\x1b[0m";
        assert_eq!(str.len(), visible_width(ansi_str))
    }
}
