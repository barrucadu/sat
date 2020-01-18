//! The empty theory.  Instantiate this to get a SAT solver.

use crate::cnf::Literal;
use crate::theory::Theory;

/// The empty theory has no state.
#[derive(Default)]
pub struct Empty();

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

impl Theory for Empty {
    fn decide(&self, _lit: Literal) -> Option<bool> {
        None
    }

    fn incorporate(&mut self, _lit: Literal) {}

    fn forget(&mut self) {}
}
