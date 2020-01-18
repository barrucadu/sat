//! Parser combinators.

use std::iter::Peekable;
use std::str::Chars;

/// Remove all following whitespace.
pub fn eat_whitespace<'a>(chars: &mut Peekable<Chars<'a>>) {
    while let Some(' ') = chars.peek() {
        chars.next();
    }
}

/// Parse a nonempty usize.
pub fn parse_usize<'a>(chars: &mut Peekable<Chars<'a>>) -> Option<usize> {
    let mut out: usize = 0;
    let mut first = true;

    while let Some(c) = chars.peek() {
        if let Some(d) = c.to_digit(10) {
            chars.next();
            out = out * 10 + (d as usize);
            first = false;
        } else {
            break;
        }
    }

    if first {
        None
    } else {
        Some(out)
    }
}
