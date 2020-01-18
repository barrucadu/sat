use std::env::var;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Result, Write};

/// A test case
pub struct Case {
    pub name: String,
    is_sat: bool,
    output_filename: String,
    expected: String,
}

/// Get the test files for a theory.
pub fn get_tests_for(theory: &str, is_sat: bool) -> Vec<(Case, String)> {
    let input_path = format!(
        "{}/tests/data/{}/{}/inputs",
        env!("CARGO_MANIFEST_DIR"),
        theory,
        if is_sat { "sat" } else { "unsat" }
    );
    let output_path = format!(
        "{}/tests/data/{}/{}/outputs",
        env!("CARGO_MANIFEST_DIR"),
        theory,
        if is_sat { "sat" } else { "unsat" }
    );

    let mut out = Vec::new();

    for entry in fs::read_dir(&input_path).unwrap() {
        let e = entry.unwrap();
        if e.metadata().unwrap().is_file() {
            if let Ok(filename) = e.file_name().into_string() {
                let input_filename = format!("{}/{}", input_path, filename);
                let output_filename = format!("{}/{}", output_path, filename);
                out.push((
                    Case {
                        name: filename.clone(),
                        output_filename: output_filename.clone(),
                        expected: read_file(output_filename).unwrap(),
                        is_sat,
                    },
                    read_file(input_filename).unwrap(),
                ));
            }
        }
    }

    out
}

/// Check a test output, or regenerate the output if
/// REGENERATE_OUTPUTS is set.
pub fn check_or_regenerate<D: Debug>(case: Case, result: Option<D>) {
    match (case.is_sat, result.is_some()) {
        (true, true) => (),
        (true, false) => panic!("expected SAT but got UNSAT"),
        (false, true) => panic!("expected UNSAT but got SAT"),
        (false, false) => (),
    }

    let actual = format!("{:?}\n", result);

    if var("REGENERATE_OUTPUTS").is_ok() {
        if case.expected != actual {
            write_file(case.output_filename, actual).unwrap();
        }
    } else {
        assert_eq!(case.expected, actual);
    }
}

/// Read a file to a string.
fn read_file(path: String) -> Result<String> {
    let file = File::open(path)?;
    let mut buffer = String::new();
    BufReader::new(file).read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Write a string to a file.
fn write_file(path: String, contents: String) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
