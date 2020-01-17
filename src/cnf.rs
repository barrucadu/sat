//! Types and utility functions for conjunctive normal form.

use std::fmt;

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
    pub fn is_negated(self) -> bool {
        let Literal(atom) = self;
        atom < 0
    }

    /// Negate a literal, with double negation cancelling out.
    pub fn negate(self) -> Literal {
        let Literal(atom) = self;
        Literal(-atom)
    }

    /// Get the numeric ID of a literal.
    pub fn get_id(self) -> isize {
        let Literal(atom) = self;
        atom.abs()
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Literal(atom) = self;
        write!(f, "{}", atom)
    }
}

/// A clause is a disjunction of literals.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Clause(pub Vec<Literal>);

impl Clause {
    /// Construct a new clause from numeric literals.
    pub fn new(lits: Vec<isize>) -> Clause {
        Clause(lits.into_iter().map(Literal::new).collect())
    }

    /// Add a literal to a clause.  Two literals corresponding to the
    /// same atom, but in positive and negative forms, can exist in
    /// the same clause.
    pub fn insert_literal(&mut self, lit: Literal) {
        let Clause(lits) = self;
        lits.push(lit);
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let Clause(lits) = self;
        if let Some((first, rest)) = lits.split_first() {
            write!(f, "{}", first)?;
            for lit in rest {
                write!(f, " || {}", lit)?;
            }
        }
        write!(f, ")")
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

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Formula(clauses) = self;
        if let Some((first, rest)) = clauses.split_first() {
            write!(f, "{}", first)?;
            for clause in rest {
                write!(f, " && {}", clause)?;
            }
        }
        Ok(())
    }
}
