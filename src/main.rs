extern crate sat;

use sat::cnf::*;
use sat::theory::empty::Empty;
use sat::theory::euf::EUF;
use sat::theory::Theory;

use std::env;
use std::fmt::Display;
use std::io::{self, Read};
use std::process::exit;

const EXIT_SAT: i32 = 0;
const EXIT_UNSAT: i32 = 1;
const EXIT_ERROR: i32 = 254;

fn main() {
    let default_theory = "sat".to_string();
    let theory_name = env::args().nth(1).unwrap_or(default_theory);

    if theory_name == "sat" {
        let (mut theory, formula) = parse_from_stdin(Empty::from_string);
        smt_main(&mut theory, formula);
    } else if theory_name == "euf" {
        let (mut theory, formula) = parse_from_stdin(EUF::from_string);
        smt_main(&mut theory, formula);
    } else {
        die("Unknown theory:", theory_name, Some("Expected 'sat'"))
    }
}

fn smt_main<T: Theory>(theory: &mut T, formula: Formula) {
    if let Some(lits) = sat::smt_assignment(theory, formula) {
        for lit in lits {
            println!("{}", lit);
        }
        exit(EXIT_SAT);
    } else {
        println!("Unsatisfiable!");
        exit(EXIT_UNSAT);
    }
}

fn parse_from_stdin<E: Display, A>(parser: fn(String) -> Result<A, E>) -> A {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => match parser(buffer) {
            Ok(a) => a,
            Err(e) => die("Failed to parse input:", e, None),
        },
        Err(e) => die("Failed to parse input:", e, None),
    }
}

fn die<T: Display>(msg: &str, e: T, ohint: Option<&str>) -> ! {
    eprintln!("{}", msg);
    eprintln!("    {}", e);
    if let Some(hint) = ohint {
        eprintln!("");
        eprintln!("{}", hint);
    }

    exit(EXIT_ERROR);
}
