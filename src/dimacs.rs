//! Parser for DIMACS CNF format

use std::fmt;

use crate::cnf::*;

/// Parse a string in DIMACS CNF format.
pub fn from_string(dimacs: String) -> Result<Formula, ParseError> {
    let mut in_prelude = true;
    let mut expected_number_of_variables = 0;
    let mut expected_number_of_clauses = 0;
    let mut clause = Vec::new();
    let mut clauses = Vec::new();
    let mut variables = 0;

    'outer: for line in dimacs.lines() {
        let mut words = line.split_ascii_whitespace();
        if in_prelude {
            match words.next() {
                Some("c") => continue,
                Some("p") => match words.next() {
                    Some("cnf") => match words.next().map(|w| w.parse::<usize>()) {
                        Some(Ok(num_vars)) => match words.next().map(|n| n.parse::<usize>()) {
                            Some(Ok(num_clauses)) => {
                                expected_number_of_variables = num_vars;
                                expected_number_of_clauses = num_clauses;
                                in_prelude = false;
                            }
                            _ => return Err(ParseError::CannotParsePreludeLine(line.to_string())),
                        },
                        _ => return Err(ParseError::CannotParsePreludeLine(line.to_string())),
                    },
                    Some(fmt) => return Err(ParseError::UnexpectedFormat(fmt.to_string())),
                    None => return Err(ParseError::CannotParsePreludeLine(line.to_string())),
                },
                _ => return Err(ParseError::CannotParsePreludeLine(line.to_string())),
            }
        } else {
            while let Some(lit) = words.next() {
                match lit.parse::<isize>() {
                    Ok(0) => {
                        clauses.push(Clause::new(clause));
                        if clauses.len() == expected_number_of_clauses {
                            break 'outer;
                        }
                        clause = Vec::new();
                    }
                    Ok(n) => {
                        let var = n.abs() as usize;
                        if var > variables {
                            variables = var;
                        }
                        clause.push(n);
                    }
                    Err(_) => return Err(ParseError::CannotParseClauseLine(line.to_string())),
                }
            }
        }
    }

    if variables == expected_number_of_variables {
        if clauses.len() == expected_number_of_clauses {
            Ok(Formula::new(clauses))
        } else {
            Err(ParseError::WrongNumberOfClauses {
                expected: expected_number_of_clauses,
                actual: clauses.len(),
            })
        }
    } else {
        Err(ParseError::WrongNumberOfVariables {
            expected: expected_number_of_variables,
            actual: variables,
        })
    }
}

/// A parser error.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ParseError {
    CannotParsePreludeLine(String),
    CannotParseClauseLine(String),
    UnexpectedFormat(String),
    WrongNumberOfVariables { expected: usize, actual: usize },
    WrongNumberOfClauses { expected: usize, actual: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::CannotParsePreludeLine(s) => write!(f, "cannot parse prelude line '{}'", s),
            ParseError::CannotParseClauseLine(s) => write!(f, "cannot parse clause line '{}'", s),
            ParseError::UnexpectedFormat(s) => write!(f, "unexpected format '{}'", s),
            ParseError::WrongNumberOfVariables { expected, actual } => write!(
                f,
                "wrong number of variables, expected {} but got {}",
                expected, actual
            ),
            ParseError::WrongNumberOfClauses { expected, actual } => write!(
                f,
                "wrong number of clauses, expected {} but got {}",
                expected, actual
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::from_string;
    use crate::cnf::*;

    #[test]
    fn works() {
        let formula = Formula::new(vec![
            Clause::new(vec![-3, 4]),
            Clause::new(vec![-1, -3, -5]),
            Clause::new(vec![-2, -4, -5]),
            Clause::new(vec![-2, 3, 5, -6]),
            Clause::new(vec![-1, 2]),
            Clause::new(vec![-1, 3, -5, -6]),
            Clause::new(vec![1, -6]),
            Clause::new(vec![1, 7]),
        ]);

        let formula_str = "c hello world\n\
                           p cnf 7 8\n\
                           -3 4 0\n\
                           -1 -3 -5 0\n\
                           -2 -4 -5 0\n\
                           -2 3 5 -6 0\n\
                           -1 2 0\n\
                           -1 3 -5 -6 0\n\
                           1 -6 0\n\
                           1 7 0";

        assert_eq!(Ok(formula), from_string(formula_str.to_string()));
    }

    #[test]
    fn works_with_awkward_newlines() {
        let formula = Formula::new(vec![
            Clause::new(vec![-3, 4]),
            Clause::new(vec![-1, -3, -5]),
            Clause::new(vec![-2, -4, -5]),
            Clause::new(vec![-2, 3, 5, -6]),
            Clause::new(vec![-1, 2]),
            Clause::new(vec![-1, 3, -5, -6]),
            Clause::new(vec![1, -6]),
            Clause::new(vec![1, 7]),
        ]);

        let formula_str = "c hello world\n\
                           p cnf 7 8\n\
                           -3 4 0 -1 -3 -5 0 -2 -4\n\
                           -5 0 -2 3 5 -6 0\n\
                           -1 2 0 -1 3 -5\n\
                           -6 0 1 -6 0 1 7 0";

        assert_eq!(Ok(formula), from_string(formula_str.to_string()));
    }

    #[test]
    fn counts_variables() {
        let formula_str = "c hello world\n\
                           p cnf 1 8\n\
                           -3 4 0\n\
                           -1 -3 -5 0\n\
                           -2 -4 -5 0\n\
                           -2 3 5 -6 0\n\
                           -1 2 0\n\
                           -1 3 -5 -6 0\n\
                           1 -6 0\n\
                           1 7 0";

        assert!(from_string(formula_str.to_string()).is_err());
    }

    #[test]
    fn counts_clauses() {
        let formula_str = "c hello world\n\
                           p cnf 7 99\n\
                           -3 4 0\n\
                           -1 -3 -5 0\n\
                           -2 -4 -5 0\n\
                           -2 3 5 -6 0\n\
                           -1 2 0\n\
                           -1 3 -5 -6 0\n\
                           1 -6 0\n\
                           1 7 0";

        assert!(from_string(formula_str.to_string()).is_err());
    }
}
