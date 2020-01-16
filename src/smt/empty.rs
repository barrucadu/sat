//! The empty theory.  Instantiate this to get a SAT solver.

use crate::cnf::Literal;
use crate::dpll::Model;
use crate::smt::Theory;

/// The empty theory has no state.
pub struct EmptyTheory();

impl EmptyTheory {
    pub fn new() -> EmptyTheory {
        EmptyTheory {}
    }
}

impl Theory for EmptyTheory {
    fn decide(&self, _model: &Model, _lit: &Literal) -> Option<bool> {
        None
    }
}
