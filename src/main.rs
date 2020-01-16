extern crate sat;

use sat::cnf::*;
use sat::dimacs;

use std::io::{self, Read};
use std::process::exit;

const EXIT_SAT: i32 = 0;
const EXIT_UNSAT: i32 = 1;
const EXIT_ERROR: i32 = 254;

fn main() {
    let formula = read_dimacs_from_stdin();

    if let Some(lits) = sat::sat_assignment(formula) {
        for lit in lits {
            println!("{}", lit);
        }
        exit(EXIT_SAT);
    } else {
        println!("Unsatisfiable!");
        exit(EXIT_UNSAT);
    }
}

fn read_dimacs_from_stdin() -> Formula {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => match dimacs::from_string(buffer) {
            Ok(formula) => formula,
            Err(e) => {
                eprintln!("Failed to parse DIMACS from stdin:");
                eprintln!("    {}", e);
                exit(EXIT_ERROR);
            }
        },
        Err(e) => {
            eprintln!("Failed to read input from stdin:");
            eprintln!("    {}", e);
            exit(EXIT_ERROR);
        }
    }
}
