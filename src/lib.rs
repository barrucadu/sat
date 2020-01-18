//! A basic SAT solver based on the paper "Abstract DPLL and Abstract
//! DPLL Modulo Theories"

pub mod cnf;
pub mod dimacs;
pub mod dpll;
pub mod theory;

use crate::cnf::{Formula, Literal};
use crate::dpll::dpll;
use crate::theory::empty::EmptyTheory;
use crate::theory::Theory;

pub fn sat(formula: Formula) -> bool {
    smt(&mut EmptyTheory::new(), formula)
}

pub fn sat_assignment(formula: Formula) -> Option<Vec<Literal>> {
    smt_assignment(&mut EmptyTheory::new(), formula)
}

pub fn smt<T: Theory>(theory: &mut T, formula: Formula) -> bool {
    dpll(theory, formula).is_some()
}

pub fn smt_assignment<T: Theory>(theory: &mut T, formula: Formula) -> Option<Vec<Literal>> {
    dpll(theory, formula).map(|model| model.get_assignments())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnf::*;
    use crate::theory::euf::*;

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

    #[test]
    fn euf_unsat_atoms() {
        let formula = Formula::new(vec![
            Clause::new(vec![1]),
            Clause::new(vec![2]),
            Clause::new(vec![3]),
            Clause::new(vec![-4]),
        ]);
        let mut euf = EUF::new(vec![
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                EUFTerm::atom(1),
            ),
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(2)]),
                EUFTerm::ap(2, vec![EUFTerm::atom(1)]),
            ),
            EUFLiteral::new(
                EUFTerm::ap(
                    1,
                    vec![
                        EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                        EUFTerm::atom(2),
                    ],
                ),
                EUFTerm::atom(3),
            ),
            EUFLiteral::new(EUFTerm::atom(1), EUFTerm::atom(3)),
        ]);

        assert!(sat(formula.clone()));
        assert!(!smt(&mut euf, formula));
    }

    #[test]
    fn euf_unsat_functions() {
        let formula = Formula::new(vec![
            Clause::new(vec![1]),
            Clause::new(vec![2]),
            Clause::new(vec![3]),
            Clause::new(vec![-4]),
        ]);
        let mut euf = EUF::new(vec![
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                EUFTerm::atom(1),
            ),
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(2)]),
                EUFTerm::ap(2, vec![EUFTerm::atom(1)]),
            ),
            EUFLiteral::new(
                EUFTerm::ap(
                    1,
                    vec![
                        EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                        EUFTerm::atom(2),
                    ],
                ),
                EUFTerm::atom(3),
            ),
            EUFLiteral::new(
                EUFTerm::ap(2, vec![EUFTerm::atom(1)]),
                EUFTerm::ap(2, vec![EUFTerm::atom(3)]),
            ),
        ]);

        assert!(sat(formula.clone()));
        assert!(!smt(&mut euf, formula));
    }

    #[test]
    fn euf_sat_functions() {
        let formula = Formula::new(vec![
            Clause::new(vec![1]),
            Clause::new(vec![2]),
            Clause::new(vec![3]),
            Clause::new(vec![-4, 4]),
        ]);
        let mut euf = EUF::new(vec![
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                EUFTerm::atom(1),
            ),
            EUFLiteral::new(
                EUFTerm::ap(1, vec![EUFTerm::atom(2)]),
                EUFTerm::ap(2, vec![EUFTerm::atom(1)]),
            ),
            EUFLiteral::new(
                EUFTerm::ap(
                    1,
                    vec![
                        EUFTerm::ap(1, vec![EUFTerm::atom(1), EUFTerm::atom(2)]),
                        EUFTerm::atom(2),
                    ],
                ),
                EUFTerm::atom(3),
            ),
            EUFLiteral::new(
                EUFTerm::ap(2, vec![EUFTerm::atom(1)]),
                EUFTerm::ap(2, vec![EUFTerm::atom(3)]),
            ),
        ]);

        assert!(sat(formula.clone()));
        assert!(smt(&mut euf, formula));
    }
}
