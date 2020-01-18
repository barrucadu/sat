//! Parse SAT formulae.

use crate::cnf::Formula;
use crate::parse::dimacs;
use crate::theory::empty::Empty;

pub fn from_string(input: String) -> Result<(Empty, Formula), dimacs::ParseError> {
    dimacs::from_string(input).map(|formula| (Empty::new(), formula))
}
