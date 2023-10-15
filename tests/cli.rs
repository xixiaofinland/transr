use assert_cmd::Command;
// use predicates::prelude::*;
use std::fs;
// use tempfile::NamedTempFile;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CLI: &str = "transr";

// --------------------------------------------------
#[test]
// the repo ships with csv and xml folder by default;
fn no_arg_works() -> TestResult {
    Command::cargo_bin(CLI)?.assert().success();
    Ok(())
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    let args = ["-i tests/1/input.csv", "-p tests/1/xml/", "-d"];
    let expected = "tests/1/r.xml";
    run(&args, &expected)
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(CLI)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
