use sat;
use sat::parse::empty;

mod common;

#[test]
fn test_sat() {
    for (case, input) in common::get_tests_for("empty", true) {
        eprintln!("{}", case.name);
        match empty::from_string(input) {
            Ok((mut theory, formula)) => {
                common::check_or_regenerate(case, sat::smt_assignment(&mut theory, formula));
            }
            Err(e) => panic!(e),
        }
    }
}

#[test]
fn test_unsat() {
    for (case, input) in common::get_tests_for("empty", false) {
        eprintln!("{}", case.name);
        match empty::from_string(input) {
            Ok((mut theory, formula)) => {
                common::check_or_regenerate(case, sat::smt_assignment(&mut theory, formula));
            }
            Err(e) => panic!(e),
        }
    }
}
