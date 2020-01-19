//! A basic SAT solver based on the paper "Abstract DPLL and Abstract
//! DPLL Modulo Theories"

use std::collections::BTreeSet;

use crate::cnf::*;
use crate::theory::Theory;

impl Literal {
    /// A literal is true in a model if it's a member of the set.
    /// This will return 'None' if the model doesn't have an
    /// assignment of truth for the literal or its negation.
    pub fn is_true_in(self, model: &Model) -> Option<bool> {
        if model.contains(self) {
            Some(true)
        } else if model.contains(self.negate()) {
            Some(false)
        } else {
            None
        }
    }
}

impl Clause {
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

impl Formula {
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
pub struct Model(Vec<(Literal, Provenance)>);

/// Literals in a model track where they've come from: this is because
/// backtracking is done in terms of literals arising from decisions.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Provenance {
    UnitPropagation,
    TheoryPropagation,
    Decision,
    Backjump,
}

impl Model {
    /// Construct a new empty model.
    fn new() -> Model {
        Model(vec![])
    }

    /// Append a literal to a model.
    fn append(&mut self, lit: Literal, provenance: Provenance) {
        let Model(lits) = self;
        lits.push((lit, provenance));
    }

    /// Check if the model contains a literal.
    fn contains(&self, lit: Literal) -> bool {
        let Model(lits) = self;

        for (l, _) in lits {
            if *l == lit {
                return true;
            }
        }

        false
    }

    /// Get the true literals from the model, discarding the
    /// provenance information.
    pub fn get_assignments(&self) -> Vec<Literal> {
        let Model(lits) = self;
        lits.iter().map(|(l, _)| *l).collect()
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

fn do_unit_propagation(model: &mut Model, formula: &Formula) -> Option<Literal> {
    let Formula(clauses) = formula;

    for clause in clauses {
        if clause.is_true_in(model) == None {
            let Clause(lits) = clause;

            for lit in lits {
                if lit.is_true_in(model) == None {
                    let lits_without_lit = lits.iter().filter(|l| *l != lit).copied().collect();

                    if Clause(lits_without_lit).is_true_in(model) == Some(false) {
                        return Some(*lit);
                    }
                }
            }
        }
    }

    None
}

fn do_theory_propagation<T: Theory>(
    theory: &T,
    model: &mut Model,
    formula: &Formula,
) -> Option<Literal> {
    let Formula(clauses) = formula;

    for clause in clauses {
        let Clause(lits) = clause;
        for lit in lits {
            if lit.is_true_in(model) == None {
                match theory.decide(*lit) {
                    Some(true) => {
                        return Some(*lit);
                    }
                    Some(false) => {
                        return Some(lit.negate());
                    }
                    None => continue,
                }
            }
        }
    }

    None
}

fn do_decision(model: &mut Model, formula: &Formula) -> Option<Literal> {
    let Formula(clauses) = formula;

    for clause in clauses {
        if clause.is_true_in(model) == None {
            let Clause(lits) = clause;

            for lit in lits {
                if lit.is_true_in(model) == None {
                    // make the lit positive
                    return Some(Literal::new(lit.get_id()));
                }
            }
        }
    }

    None
}

fn reset_theory<T: Theory>(theory: &mut T, model: &Model) {
    let Model(lits) = model;

    theory.forget();

    for (lit, _) in lits.iter() {
        theory.incorporate(*lit);
    }
}

/// Given a formula, find a model which satisfies it if one exists.
pub fn dpll<T: Theory>(theory: &mut T, formula0: Formula) -> Option<Model> {
    let mut model = Model::new();
    let formula = simplify_formula(formula0);

    loop {
        match formula.is_true_in(&model) {
            Some(true) => break,
            Some(false) => {
                if do_backjump(&mut model) {
                    reset_theory(theory, &model);
                    continue;
                }

                return None;
            }
            None => {
                // need to eagerly apply constraints required by the
                // theory, or unit propagation might pick a literal
                // which the theory would forbid.
                if let Some(lit) = do_theory_propagation(theory, &mut model, &formula) {
                    model.append(lit, Provenance::TheoryPropagation);
                    theory.incorporate(lit);
                    continue;
                }
                if let Some(lit) = do_unit_propagation(&mut model, &formula) {
                    model.append(lit, Provenance::UnitPropagation);
                    theory.incorporate(lit);
                    continue;
                }
                if let Some(lit) = do_decision(&mut model, &formula) {
                    model.append(lit, Provenance::Decision);
                    theory.incorporate(lit);
                    continue;
                }

                panic!("failed to do either propagation or decision in an incomplete model");
            }
        }
    }

    Some(model)
}

// "Clause Elimination Procedures for CNF Formulas" - Heule, Jarvisalo, Biere (2010)

fn simplify_formula(formula: Formula) -> Formula {
    // maybe one day there will be more than just one
    asymmetric_tautology_elimination(formula)
}

/// Asymmetric Tautology Elimination (4.2)
///
/// For a clause C and a CNF formula F, ALA(F,C) denotes the unique
/// clause resulting from repeating the following until fixpoint:
/// if l1, ..., lk \in C and there is a clause (l1 || ... || lk ||
/// l) \in F \ {C} for some literal l, let C := C + {!l}.
///
/// A clause C is called an asymmetric tautology if and only if
/// ALA(F,C) is a tautology.
///
/// Given a formula F, asymmetric tautology elimination (ATE)
/// repeats the following until fixpoint: if there is an asymmetric
/// tautological clause C \in F, let F := F \ {C}
fn asymmetric_tautology_elimination(formula: Formula) -> Formula {
    let Formula(clauses) = formula;
    let mut deleted_clauses: Vec<bool> = clauses.iter().map(|_| false).collect();

    for i in 0..clauses.len() {
        let Clause(lits) = &clauses[i];
        let mut ate_lits = lits.clone();
        'ala: loop {
            let ate_lits_set: BTreeSet<Literal> = ate_lits.iter().copied().collect();
            for j in 0..clauses.len() {
                if i == j {
                    continue;
                }
                if deleted_clauses[j] {
                    continue;
                }
                let Clause(elits) = &clauses[j];
                for el in elits {
                    if ate_lits_set.contains(&el.negate()) {
                        continue;
                    }
                    let mut els_without_el: BTreeSet<Literal> = elits.iter().copied().collect();
                    els_without_el.remove(el);
                    if els_without_el.is_subset(&ate_lits_set) {
                        if ate_lits_set.contains(&el) {
                            deleted_clauses[i] = true;
                            break 'ala;
                        } else {
                            ate_lits.push(el.negate());
                            continue 'ala;
                        }
                    }
                }
            }
            break 'ala;
        }
    }

    Formula(
        clauses
            .iter()
            .zip(deleted_clauses.iter())
            .filter(|(_, deleted)| !**deleted)
            .map(|(clause, _)| clause)
            .cloned()
            .collect(),
    )
}
