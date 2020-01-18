//! Parse EUF formulae.

use std::fmt;
use std::iter::Peekable;

use crate::cnf::Formula;
use crate::parse::combinators::*;
use crate::parse::dimacs;
use crate::theory::euf::*;

/// Parse an EUF theory and formula represented as a string.  The
/// representation is as follows:
///
///    euf_lit
///    euf_lit
///    [...]
///    ---
///    <dimacs>
///
/// Where an euf_lit is one of:
///   - == euf_term euf_term
///   - /= euf_term euf_term
///
/// Where an euf_term is one of:
///   - integer
///   - integer(euf_term...)
pub fn from_string(input: String) -> Result<(EUF, Formula), ParseError> {
    let mut lines = input.lines();
    let mut lits = Vec::new();

    while let Some(line) = lines.next() {
        if line == "--" {
            break;
        } else {
            match parse_lit(&mut line.chars().peekable()) {
                Ok(lit) => lits.push(lit),
                Err(e) => return Err(e),
            }
        }
    }

    match dimacs::from_lines(lines) {
        Ok(formula) => Ok((EUF::new(lits), formula)),
        Err(e) => Err(ParseError::DIMACSError(e)),
    }
}

/// Parse a lit, one of:
///   - == euf_term euf_term
///   - /= euf_term euf_term
fn parse_lit<'a>(mut chars: &mut Peekable<std::str::Chars<'a>>) -> Result<EUFLiteral, ParseError> {
    let c1 = chars.next();
    let c2 = chars.next();

    let is_equality = match (c1, c2) {
        (Some('='), Some('=')) => true,
        (Some('/'), Some('=')) => false,
        _ => return Err(ParseError::CannotParseEqualitySymbol { c1, c2 }),
    };

    let left = parse_term(&mut chars)?;
    let right = parse_term(&mut chars)?;

    let lit = EUFLiteral::new(left, right);
    if is_equality {
        Ok(lit)
    } else {
        Ok(lit.negate())
    }
}

/// Parse a term, is one of:
///   - integer
///   - integer(euf_term...)
fn parse_term<'a>(mut chars: &mut Peekable<std::str::Chars<'a>>) -> Result<EUFTerm, ParseError> {
    eat_whitespace(&mut chars);

    let atom = parse_atom(&mut chars)?;
    let mut parameters = Vec::new();

    eat_whitespace(&mut chars);

    match chars.peek() {
        Some('(') => {
            chars.next();
            loop {
                match chars.peek() {
                    Some(')') => {
                        chars.next();
                        break;
                    }
                    None => return Err(ParseError::UnexpectedEndOfApTerm),
                    _ => (),
                }
                let term = parse_term(&mut chars)?;
                parameters.push(term);
                eat_whitespace(&mut chars);
            }
            Ok(EUFTerm::ap(atom, parameters))
        }
        _ => Ok(EUFTerm::atom(atom)),
    }
}

/// Parse an atom
fn parse_atom<'a>(mut chars: &mut Peekable<std::str::Chars<'a>>) -> Result<usize, ParseError> {
    if let Some(atom) = parse_usize(&mut chars) {
        Ok(atom)
    } else {
        Err(ParseError::CannotParseAtom)
    }
}

/// A parser error.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ParseError {
    CannotParseEqualitySymbol { c1: Option<char>, c2: Option<char> },
    UnexpectedEndOfApTerm,
    CannotParseAtom,
    DIMACSError(dimacs::ParseError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::CannotParseEqualitySymbol { c1, c2 } => match (c1, c2) {
                (Some(a), Some(b)) => write!(
                    f,
                    "cannot parse equality symbol, expected '==' or '/=' but got '{}{}'",
                    a, b
                ),
                (Some(a), None) => write!(
                    f,
                    "cannot parse equality symbol, expected '==' or '/=' but got '{}'",
                    a
                ),
                _ => write!(f, "unexpected empty line"),
            },
            ParseError::UnexpectedEndOfApTerm => write!(f, "unexpected end of application term"),
            ParseError::CannotParseAtom => write!(f, "cannot parse atom"),
            ParseError::DIMACSError(e) => write!(f, "cannot parse DIMACS: {}", e),
        }
    }
}
