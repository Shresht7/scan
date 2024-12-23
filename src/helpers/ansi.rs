/// Character that denotes the starts of escape codes
const ESC: char = '\x1b';

/// Determine the length of characters in an ANSI string that are visible in the terminal
pub fn visible_width(s: &str) -> usize {
    let mut width: usize = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        // If we have not encountered a ESC yet ...
        if c != ESC {
            width += 1 // ... keep adding 1 to the width (char count)
        } else {
            // .. otherwise, we hit the start of an ESC sequence
            'esc_sequence: while let Some(c) = chars.next() {
                match c {
                    // Control Sequence Introducer: continue until `\x40-\x7E` (ASCII @A–Z[\]^_`a–z{|}~).
                    // See https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences
                    '[' => {
                        loop {
                            let next = chars.next();
                            if matches!(next, Some('\x40'..='\x7E') | None) {
                                break 'esc_sequence; // Break as soon as we encounter the end of an ansi-code
                            }
                        }
                    }
                    _ => width += 1, // If outside a control sequence, continue adding 1 to the width (char count)
                }
            }
        }
    }

    width
}

/// Truncate the given string to the given width accounting for the ANSI escape sequences
pub fn truncate_visible(s: &str, width: usize) -> String {
    let mut trunc = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        trunc.push(c);
        // TODO: Calling visible_width over and over is extremely silly
        // Adapt the loop above to perform the truncation in one go
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
