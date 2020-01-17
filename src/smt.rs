//! Theories

pub mod empty;
pub mod euf;

use crate::cnf::Literal;

/// A trait for theories, allowing you to implement your own.  See
/// submodules of sat::smt:: for theories this solver comes with.
pub trait Theory {
    /// Decide the truth value of a literal in a model under the
    /// theory, if possible.
    fn decide(&self, lit: &Literal) -> Option<bool>;

    /// Add a new literal to the theory.  This will only be called if
    /// self.decide(lit) is Some(true) or None.
    fn incorporate(&mut self, lit: &Literal);

    /// Forget all literals (used for backjumping).
    fn forget(&mut self);
}
