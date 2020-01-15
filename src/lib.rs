//! A basic SAT solver based on the paper "Abstract DPLL and Abstract
//! DPLL Modulo Theories"

use std::collections::BTreeSet;

pub const HELLO_WORLD: &str = "Hello, world!";

/// An atom is a propositional symbol, represented here by integers.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Atom(usize);

/// A literal is either an atom or the negation of that atom.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Literal {
    atom: Atom,
    is_negative: bool,
}

impl Literal {
    /// Construct a positive literal from an atom.
    pub fn new(atom: Atom) -> Literal {
        Literal {
            atom: atom,
            is_negative: false,
        }
    }

    /// Negate a literal, with double negation cancelling out.
    pub fn negate(&self) -> Literal {
        Literal {
            atom: self.atom,
            is_negative: !self.is_negative,
        }
    }

    /// A literal is true in a model if it's a member of the set.
    /// This will return 'None' if the model doesn't have an
    /// assignment of truth for the literal or its negation.
    pub fn is_true_in(&self, model: &Model) -> Option<bool> {
        if model.contains(self) {
            Some(true)
        } else if model.contains(&self.negate()) {
            Some(false)
        } else {
            None
        }
    }
}

/// A clause is a disjunction of literals.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Clause(BTreeSet<Literal>);

impl Clause {
    /// Construct a new clause from a literal.
    pub fn new(lit: Literal) -> Clause {
        let mut lits = BTreeSet::new();
        lits.insert(lit);
        Clause(lits)
    }

    /// Add a literal to a clause.  Two literals corresponding to the
    /// same atom, but in positive and negative forms, can exist in
    /// the same clause.
    pub fn insert_literal(&mut self, lit: Literal) {
        let Clause(lits) = self;
        lits.insert(lit);
    }

    /// A clause is true in a model if any of its literals are true in
    /// the model.  This will return 'None' if none of the literals
    /// have their truth decided by the model.
    pub fn is_true_in(&self, model: &Model) -> Option<bool> {
        let Clause(lits) = self;
        let mut all_false = true;

        for lit in lits {
            match lit.is_true_in(model) {
                Some(true) => return Some(true),
                Some(false) => continue,
                None => all_false = false,
            }
        }

        if all_false {
            Some(false)
        } else {
            None
        }
    }
}

/// A formula is a conjunction of clauses.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Formula(BTreeSet<Clause>);

impl Formula {
    /// Construct a new formula from a clause.
    pub fn new(clause: Clause) -> Formula {
        let mut clauses = BTreeSet::new();
        clauses.insert(clause);
        Formula(clauses)
    }

    /// Add a clause to a formula.
    pub fn insert_clause(&mut self, clause: Clause) {
        let Formula(clauses) = self;
        clauses.insert(clause);
    }

    /// A formula is true in a model if all of its clauses are true in
    /// the model.  This will return 'None' if at least one of the
    /// clauses doesn't have its truth determined by the model.
    pub fn is_true_in(&self, model: &Model) -> Option<bool> {
        let Formula(clauses) = self;

        for clause in clauses {
            match clause.is_true_in(model) {
                Some(true) => continue,
                Some(false) => return Some(false),
                None => return None,
            }
        }

        Some(true)
    }
}

/// A model, or partial truth assignment, is a set of literals which
/// are true.  Implemented as a vec because the DPLL algorithm makes
/// use of the order of assignments when backtracking.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Model(Vec<(Literal, Provenance)>);

/// Literals in a model track where they've come from: this is because
/// backtracking is done in terms of literals arising from decisions.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Provenance {
    UnitPropagation,
    Decision,
    Backjump,
}

impl Model {
    /// Construct a new empty model.
    pub fn new() -> Model {
        Model(vec![])
    }

    /// Append a literal to a model.
    pub fn append(&mut self, lit: Literal, provenance: Provenance) {
        let Model(lits) = self;
        lits.push((lit, provenance));
    }

    /// Check if the model contains a literal.
    pub fn contains(&self, lit: &Literal) -> bool {
        let Model(lits) = self;

        for (l, _) in lits {
            if l == lit {
                return true;
            }
        }

        false
    }

    /// Check if the model is consistent: there exists no literal L
    /// such that both L and L.negate are in the model.
    pub fn is_consistent(&self) -> bool {
        let mut has_positive = BTreeSet::new();
        let mut has_negative = BTreeSet::new();

        let Model(lits) = self;

        for (lit, _) in lits {
            if lit.is_negative {
                has_negative.insert(lit.atom);
            } else {
                has_positive.insert(lit.atom);
            }
        }

        has_positive.intersection(&has_negative).count() == 0
    }
}

fn do_backjump(model: &mut Model) -> bool {
    // this is a simpler version than in the paper (what it calls
    // "standard backtracking") because I'm not sure how to implement
    // `F |= C \/ l'`
    let Model(lits) = model;

    while let Some((lit, provenance)) = lits.pop() {
        if provenance == Provenance::Decision {
            model.append(lit.negate(), Provenance::Backjump);
            return true;
        }
    }

    false
}

fn do_unit_propagation(model: &mut Model, formula: &Formula) -> bool {
    let Formula(clauses) = formula;

    for clause in clauses {
        if clause.is_true_in(model) == None {
            let Clause(lits) = clause;

            for lit in lits {
                if lit.is_true_in(model) == None {
                    let mut lits_without_lit = lits.clone();
                    lits_without_lit.remove(lit);

                    if Clause(lits_without_lit).is_true_in(model) == Some(false) {
                        model.append(*lit, Provenance::UnitPropagation);
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn do_decision(model: &mut Model, formula: &Formula) -> bool {
    let Formula(clauses) = formula;

    for clause in clauses {
        if clause.is_true_in(model) == None {
            let Clause(lits) = clause;

            for lit in lits {
                if lit.is_true_in(model) == None {
                    // make the lit positive
                    model.append(Literal::new(lit.atom), Provenance::Decision);
                    return true;
                }
            }
        }
    }

    false
}

fn dpll(formula: Formula) -> Option<Model> {
    let mut model = Model::new();

    loop {
        match formula.is_true_in(&model) {
            Some(true) => break,
            Some(false) => {
                if do_backjump(&mut model) {
                    continue;
                }

                return None;
            }
            None => {
                if do_unit_propagation(&mut model, &formula) {
                    continue;
                }
                if do_decision(&mut model, &formula) {
                    continue;
                }

                panic!("failed to do either unit propagation or decision in an incomplete model");
            }
        }
    }

    Some(model)
}

fn sat(formula: Formula) -> bool {
    dpll(formula).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sat_1() {
        assert!(sat(mk_formula(&mut vec![mk_clause(&mut vec![1])])));
    }

    #[test]
    fn simple_sat_2() {
        assert!(sat(mk_formula(&mut vec![mk_clause(&mut vec![1, 2])])));
    }

    #[test]
    fn simple_sat_2b() {
        assert!(sat(mk_formula(&mut vec![
            mk_clause(&mut vec![-1]),
            mk_clause(&mut vec![1, -2]),
        ])));
    }

    #[test]
    fn simple_sat_3() {
        assert!(sat(mk_formula(&mut vec![
            mk_clause(&mut vec![1, 2]),
            mk_clause(&mut vec![3])
        ])));
    }

    #[test]
    fn simple_unsat_1() {
        assert!(!sat(mk_formula(&mut vec![
            mk_clause(&mut vec![1]),
            mk_clause(&mut vec![-1])
        ])));
    }

    #[test]
    fn simple_unsat_2() {
        assert!(!sat(mk_formula(&mut vec![
            mk_clause(&mut vec![1]),
            mk_clause(&mut vec![2]),
            mk_clause(&mut vec![-1, -2]),
        ])));
    }

    #[test]
    fn complex_sat_7() {
        assert!(sat(mk_formula(&mut vec![
            mk_clause(&mut vec![-3, 4]),
            mk_clause(&mut vec![-1, -3, -5]),
            mk_clause(&mut vec![-2, -4, -5]),
            mk_clause(&mut vec![-2, 3, 5, -6]),
            mk_clause(&mut vec![-1, 2]),
            mk_clause(&mut vec![-1, 3, -5, -6]),
            mk_clause(&mut vec![1, -6]),
            mk_clause(&mut vec![1, 7]),
        ])));
    }

    fn mk_lit(atom: isize) -> Literal {
        if atom < 0 {
            Literal::new(Atom((atom * -1) as usize)).negate()
        } else {
            Literal::new(Atom(atom as usize))
        }
    }

    fn mk_clause(lits: &mut Vec<isize>) -> Clause {
        if let Some(a) = lits.pop() {
            let mut clause = Clause::new(mk_lit(a));
            while let Some(b) = lits.pop() {
                clause.insert_literal(mk_lit(b));
            }
            clause
        } else {
            panic!("tried to construct empty clause")
        }
    }

    fn mk_formula(clauses: &mut Vec<Clause>) -> Formula {
        if let Some(a) = clauses.pop() {
            let mut formula = Formula::new(a);
            while let Some(b) = clauses.pop() {
                formula.insert_clause(b);
            }
            formula
        } else {
            panic!("tried to construct empty formula")
        }
    }
}
