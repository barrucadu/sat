//! The theory of equality with uninterpreted function symbols.  This
//! allows expressing problems like:
//!
//!    g(a) = c && (f(g(a)) != f(c) || g(a) = d) && c != d

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::cnf::Literal;
use crate::smt::Theory;

/// An EUF term is either an atom (represented as numbers) or a
/// function applied to an EUF term.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum EUFTerm {
    Atom(usize),
    Application {
        function_atom: usize,
        parameters: Vec<EUFTerm>,
    },
}

impl EUFTerm {
    /// Construct an EUF atom term.
    pub fn atom(atom: usize) -> EUFTerm {
        EUFTerm::Atom(atom)
    }

    /// Construct an EUF application term
    pub fn ap(function_atom: usize, parameters: Vec<EUFTerm>) -> EUFTerm {
        EUFTerm::Application {
            function_atom: function_atom,
            parameters: parameters,
        }
    }
}

/// An EUF literal is an (in)equality applied to two EUF terms.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct EUFLiteral {
    is_equality: bool,
    left: EUFTerm,
    right: EUFTerm,
}

impl EUFLiteral {
    /// Construct an EUF equality literal
    pub fn new(left: EUFTerm, right: EUFTerm) -> EUFLiteral {
        EUFLiteral {
            is_equality: true,
            left: left,
            right: right,
        }
    }
}

pub struct EUF {
    lits: Vec<EUFLiteral>,
    superterms: BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    equivs: BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    inequivs: BTreeSet<(EUFTerm, EUFTerm)>,
}

impl EUF {
    /// Construct an EUF theory for the given set of literals.  For
    /// cnf literal X, lits[X-1] should be the corresponding euf
    /// literal.
    pub fn new(lits: Vec<EUFLiteral>) -> EUF {
        let superterms = compute_superterms(&lits);
        EUF {
            lits: lits,
            superterms: superterms,
            equivs: BTreeMap::new(),
            inequivs: BTreeSet::new(),
        }
    }

    fn to_euf_lit(&self, model_lit: &Literal) -> EUFLiteral {
        let euf_lit = self.lits[(model_lit.get_id() as usize) - 1].clone();
        if model_lit.is_negated() {
            EUFLiteral {
                is_equality: !euf_lit.is_equality,
                left: euf_lit.left,
                right: euf_lit.right,
            }
        } else {
            euf_lit
        }
    }
}

impl Theory for EUF {
    fn decide(&self, model_lit: &Literal) -> Option<bool> {
        let euf_lit = self.to_euf_lit(model_lit);

        if euf_lit.left == euf_lit.right {
            return Some(euf_lit.is_equality);
        }

        match (
            euf_lit.is_equality,
            are_equal(&self.equivs, &euf_lit.left, &euf_lit.right),
            are_unequal(&self.equivs, &self.inequivs, &euf_lit.left, &euf_lit.right),
        ) {
            (true, true, false) => Some(true),
            (true, false, true) => Some(false),
            (false, true, false) => Some(false),
            (false, false, true) => Some(true),
            (_, true, true) => panic!(
                "contradiction: {:?} and {:?} are both equal and unequal",
                euf_lit.left, euf_lit.right
            ),
            (_, false, false) => None,
        }
    }

    fn incorporate(&mut self, model_lit: &Literal) {
        let el = self.to_euf_lit(model_lit);
        if el.is_equality {
            if el.left == el.right {
                return;
            }
            add_equiv(&mut self.equivs, &self.superterms, &el.left, &el.right)
        } else {
            if el.left == el.right {
                panic!("contradiction: {:?} is not equal to itself", el.left);
            }
            self.inequivs.insert((el.left.clone(), el.right.clone()));
        }

        infer_implicit_equalities(&mut self.equivs, &self.superterms);
    }

    fn forget(&mut self) {
        self.equivs = BTreeMap::new();
        self.inequivs = BTreeSet::new();
    }
}

/// Given a set of literals, compute the superterm relation.
fn compute_superterms(lits: &Vec<EUFLiteral>) -> BTreeMap<EUFTerm, BTreeSet<EUFTerm>> {
    fn go(superterms: &mut BTreeMap<EUFTerm, BTreeSet<EUFTerm>>, term: &EUFTerm) {
        superterms.entry(term.clone()).or_default();

        if let EUFTerm::Application {
            function_atom: _,
            parameters: ps,
        } = term
        {
            for p in ps {
                superterms
                    .entry(p.clone())
                    .or_default()
                    .insert(term.clone());
                go(superterms, p);
            }
        }
    }

    let mut superterms: BTreeMap<EUFTerm, BTreeSet<EUFTerm>> = BTreeMap::new();
    for lit in lits {
        go(&mut superterms, &lit.left);
        go(&mut superterms, &lit.right);
    }
    superterms
}

/// Add an equivalence relation to the proof tree.
fn add_equiv(
    rel: &mut BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    superterms: &BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    left: &EUFTerm,
    right: &EUFTerm,
) {
    fn go(
        rel: &mut BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
        superterms: &BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
        term: &EUFTerm,
        equiv: &EUFTerm,
        function_atom: usize,
        parameters: &Vec<EUFTerm>,
        prefix: Vec<EUFTerm>,
        i: usize,
        changed: bool,
    ) {
        if i == parameters.len() {
            if changed {
                let new_term = EUFTerm::Application {
                    function_atom: function_atom,
                    parameters: prefix,
                };
                // only add equivalences for terms which exist in the
                // problem.
                if !superterms.contains_key(&new_term) {
                    return;
                }
                add_equiv(
                    rel,
                    superterms,
                    &EUFTerm::Application {
                        function_atom: function_atom,
                        parameters: parameters.clone(),
                    },
                    &new_term,
                );
            }
        } else {
            let p = parameters[i].clone();
            if p == *term {
                let mut prefix_changed = prefix.clone();
                prefix_changed.push(equiv.clone());
                go(
                    rel,
                    superterms,
                    term,
                    equiv,
                    function_atom,
                    parameters,
                    prefix_changed,
                    i + 1,
                    true,
                );
            }
            let mut prefix_same = prefix.clone();
            prefix_same.push(p);
            go(
                rel,
                superterms,
                term,
                equiv,
                function_atom,
                parameters,
                prefix_same,
                i + 1,
                changed,
            );
        }
    }

    rel.entry(left.clone()).or_default().insert(right.clone());
    rel.entry(right.clone()).or_default().insert(left.clone());

    let empty_set = BTreeSet::new();
    for superterm in superterms.get(left).unwrap_or(&empty_set).into_iter() {
        match superterm {
            EUFTerm::Application {
                function_atom,
                parameters,
            } => go(
                rel,
                superterms,
                left,
                right,
                *function_atom,
                parameters,
                Vec::new(),
                0,
                false,
            ),
            _ => continue,
        }
    }
    for superterm in superterms.get(right).unwrap_or(&empty_set).into_iter() {
        match superterm {
            EUFTerm::Application {
                function_atom,
                parameters,
            } => go(
                rel,
                superterms,
                right,
                left,
                *function_atom,
                parameters,
                Vec::new(),
                0,
                false,
            ),
            _ => continue,
        }
    }
}

/// Infer any new implicit equalities.
///
/// Currently just finds new function equalities.  It might also be
/// good to compute the transitive closure of the equivalence sets, to
/// avoid tree-walking in are_equal.
fn infer_implicit_equalities(
    rel: &mut BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    superterms: &BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
) {
    loop {
        let mut new_equivalences = Vec::new();

        // find function equalities
        'fn_a: for a in superterms.keys() {
            match a {
                EUFTerm::Application {
                    function_atom: af,
                    parameters: aps,
                } => {
                    'fn_b: for b in superterms.keys() {
                        if *a == *b {
                            continue 'fn_b;
                        }
                        match b {
                            EUFTerm::Application {
                                function_atom: bf,
                                parameters: bps,
                            } if af == bf && aps.len() == bps.len() && !are_equal(rel, a, b) => {
                                for (ap, bp) in aps.into_iter().zip(bps.into_iter()) {
                                    if !are_equal(rel, ap, bp) {
                                        continue 'fn_b;
                                    }
                                }
                                // atoms, arities, and parameters are equal
                                new_equivalences.push((a.clone(), b.clone()));
                            }
                            _ => continue 'fn_b,
                        }
                    }
                }
                _ => continue 'fn_a,
            }
        }

        if new_equivalences.len() == 0 {
            break;
        }

        for (a, b) in new_equivalences {
            add_equiv(rel, superterms, &a, &b);
        }
    }
}

/// Check if two terms are known to be equal.
fn are_equal(rel: &BTreeMap<EUFTerm, BTreeSet<EUFTerm>>, left: &EUFTerm, right: &EUFTerm) -> bool {
    if *left == *right {
        return true;
    }

    let mut seen = BTreeSet::new();
    let mut todo = vec![left];
    let empty_set = BTreeSet::new();

    while let Some(next) = todo.pop() {
        {
            for candidate in rel.get(next).unwrap_or(&empty_set) {
                if seen.contains(candidate) {
                    continue;
                }
                if *candidate == *right {
                    return true;
                }
                todo.push(candidate);
            }
        }
        seen.insert(next.clone());
    }

    false
}

/// Check if two terms are known to be unequal.
fn are_unequal(
    rel: &BTreeMap<EUFTerm, BTreeSet<EUFTerm>>,
    inequivs: &BTreeSet<(EUFTerm, EUFTerm)>,
    left: &EUFTerm,
    right: &EUFTerm,
) -> bool {
    if *left == *right {
        return false;
    }

    for (a, b) in inequivs {
        if (are_equal(rel, left, a) && are_equal(rel, right, b))
            || (are_equal(rel, left, b) && are_equal(rel, right, a))
        {
            return true;
        }
    }

    false
}
