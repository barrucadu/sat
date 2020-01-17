//! The empty theory.  Instantiate this to get a SAT solver.

use crate::cnf::Literal;
use crate::smt::Theory;

/// The empty theory has no state.
#[derive(Default)]
pub struct EmptyTheory();

impl EmptyTheory {
    pub fn new() -> EmptyTheory {
        EmptyTheory {}
    }
}

impl Theory for EmptyTheory {
    fn decide(&self, _lit: Literal) -> Option<bool> {
        None
    }

    fn incorporate(&mut self, _lit: Literal) {}

    fn forget(&mut self) {}
}
