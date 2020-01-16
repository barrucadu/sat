//! A basic SAT solver based on the paper "Abstract DPLL and Abstract
//! DPLL Modulo Theories"

pub mod cnf;
pub mod dimacs;
pub mod dpll;
pub mod smt;

use crate::cnf::{Formula, Literal};
use crate::dpll::dpll;
use crate::smt::empty::EmptyTheory;
use crate::smt::Theory;

pub fn sat(formula: Formula) -> bool {
    smt(EmptyTheory::new(), formula)
}

pub fn sat_assignment(formula: Formula) -> Option<Vec<Literal>> {
    smt_assignment(EmptyTheory::new(), formula)
}

pub fn smt<T: Theory>(theory: T, formula: Formula) -> bool {
    dpll(theory, formula).is_some()
}

pub fn smt_assignment<T: Theory>(theory: T, formula: Formula) -> Option<Vec<Literal>> {
    dpll(theory, formula).map(|model| model.get_assignments())
}

#[cfg(test)]
mod tests {
    use super::sat;
    use crate::cnf::*;

    #[test]
    fn simple_sat_1() {
        assert!(sat(Formula::new(vec![Clause::new(vec![1])])));
    }

    #[test]
    fn simple_sat_2() {
        assert!(sat(Formula::new(vec![Clause::new(vec![1, 2])])));
    }

    #[test]
    fn simple_sat_2b() {
        assert!(sat(Formula::new(vec![
            Clause::new(vec![-1]),
            Clause::new(vec![1, -2]),
        ])));
    }

    #[test]
    fn simple_sat_3() {
        assert!(sat(Formula::new(vec![
            Clause::new(vec![1, 2]),
            Clause::new(vec![3])
        ])));
    }

    #[test]
    fn simple_unsat_1() {
        assert!(!sat(Formula::new(vec![
            Clause::new(vec![1]),
            Clause::new(vec![-1])
        ])));
    }

    #[test]
    fn simple_unsat_2() {
        assert!(!sat(Formula::new(vec![
            Clause::new(vec![1]),
            Clause::new(vec![2]),
            Clause::new(vec![-1, -2]),
        ])));
    }

    #[test]
    fn complex_sat_7() {
        assert!(sat(Formula::new(vec![
            Clause::new(vec![-3, 4]),
            Clause::new(vec![-1, -3, -5]),
            Clause::new(vec![-2, -4, -5]),
            Clause::new(vec![-2, 3, 5, -6]),
            Clause::new(vec![-1, 2]),
            Clause::new(vec![-1, 3, -5, -6]),
            Clause::new(vec![1, -6]),
            Clause::new(vec![1, 7]),
        ])));
    }
}
