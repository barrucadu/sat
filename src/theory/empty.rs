//! The empty theory.  Instantiate this to get a SAT solver.

use crate::cnf::{Formula, Literal};
use crate::dimacs;
use crate::theory::Theory;

/// The empty theory has no state.
#[derive(Default)]
pub struct EmptyTheory();

impl EmptyTheory {
    pub fn new() -> EmptyTheory {
        EmptyTheory {}
    }

    pub fn from_string(input: String) -> Result<(EmptyTheory, Formula), dimacs::ParseError> {
        dimacs::from_string(input).map(|formula| (EmptyTheory::new(), formula))
    }
}

impl Theory for EmptyTheory {
    fn decide(&self, _lit: Literal) -> Option<bool> {
        None
    }

    fn incorporate(&mut self, _lit: Literal) {}

    fn forget(&mut self) {}
}
