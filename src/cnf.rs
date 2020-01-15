//! Types and utility functions for conjunctive normal form.

/// A literal is either an atom (a positive number) or the negation of
/// that atom (a negative number).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Literal(pub isize);

impl Literal {
    /// Construct a positive literal from an atom.
    pub fn new(atom: isize) -> Literal {
        if atom == 0 {
            panic!("cannot construct a literal numbered zero");
        }
        Literal(atom)
    }

    /// Check if a literal is negated.
    pub fn is_negated(&self) -> bool {
        let Literal(atom) = *self;
        atom < 0
    }

    /// Negate a literal, with double negation cancelling out.
    pub fn negate(&self) -> Literal {
        let Literal(atom) = self;
        Literal(atom * -1)
    }
}

/// A clause is a disjunction of literals.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Clause(pub Vec<Literal>);

impl Clause {
    /// Construct a new clause from numeric literals.
    pub fn new(lits: Vec<isize>) -> Clause {
        Clause(lits.into_iter().map(|i| Literal::new(i)).collect())
    }

    /// Add a literal to a clause.  Two literals corresponding to the
    /// same atom, but in positive and negative forms, can exist in
    /// the same clause.
    pub fn insert_literal(&mut self, lit: Literal) {
        let Clause(lits) = self;
        lits.push(lit);
    }
}

/// A formula is a conjunction of clauses.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Formula(pub Vec<Clause>);

impl Formula {
    /// Construct a new formula from a clause.
    pub fn new(clauses: Vec<Clause>) -> Formula {
        Formula(clauses.clone())
    }

    /// Add a clause to a formula.
    pub fn insert_clause(&mut self, clause: Clause) {
        let Formula(clauses) = self;
        clauses.push(clause);
    }
}
