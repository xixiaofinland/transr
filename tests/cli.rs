use assert_cmd::Command;
// use predicates::prelude::*;
use std::fs;
// use tempfile::NamedTempFile;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CLI: &str = "transr";

// --------------------------------------------------
#[test]
// should succeed as the repo ships with csv and xml folder by default;
fn zero() -> TestResult {
    Command::cargo_bin(CLI)?.assert().success();
    Ok(())
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    let args = ["-i tests/1/input.csv", "-d"];
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
