//! Theories

pub mod empty;
pub mod euf;

use crate::cnf::Literal;
use crate::dpll::Model;

/// A trait for theories, allowing you to implement your own.  See
/// submodules of sat::smt:: for theories this solver comes with.
pub trait Theory {
    /// Decide the truth value of a literal in a model under the
    /// theory, if possible.
    fn decide(&self, model: &Model, lit: &Literal) -> Option<bool>;
}
