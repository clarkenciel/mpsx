use assert_cmd::Command;
use predicates::prelude::*;

// Helper function to create a command for our mtrue binary
fn mtrue_cmd() -> Command {
    Command::cargo_bin("mtrue").unwrap()
}

#[test]
fn test_basic_success() {
    // true should always exit with status 0
    mtrue_cmd().assert().success();
}

#[test]
fn test_no_output() {
    // true should produce no output to stdout or stderr by default
    mtrue_cmd()
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_ignores_arguments() {
    // true should ignore all command line arguments and still succeed
    mtrue_cmd()
        .args(&["random", "arguments", "here"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_ignores_unknown_flags() {
    // true should ignore unknown flags and still succeed
    mtrue_cmd()
        .args(&["--unknown-flag", "-x", "--random"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_help_flag() {
    // --help should display help and exit successfully
    mtrue_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("usage").or(predicate::str::contains("Usage")))
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_version_flag() {
    // --version should display version and exit successfully
    mtrue_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_short_help_flag() {
    // Some implementations support -h for help, but true typically ignores it
    // Based on our research, true ignores -h and still succeeds
    mtrue_cmd()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_mixed_arguments_and_flags() {
    // true should ignore mixed arguments and flags
    mtrue_cmd()
        .args(&["--help", "some", "args", "--version", "more", "args"])
        .assert()
        .success();
}

#[test]
fn test_many_arguments() {
    // true should handle many arguments gracefully
    let many_args: Vec<String> = (0..100).map(|i| format!("arg{}", i)).collect();
    mtrue_cmd()
        .args(&many_args)
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_special_characters() {
    // true should ignore arguments with special characters
    mtrue_cmd()
        .args(&["$HOME", "$(whoami)", "`date`", "foo bar", ""])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_unicode_arguments() {
    // true should ignore unicode arguments
    mtrue_cmd()
        .args(&["αβγ", "😀", "中文", "العربية"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_exit_code_consistency() {
    // Multiple runs should always return the same exit code (0)
    for _ in 0..10 {
        mtrue_cmd().assert().success();
    }
}

#[test]
fn test_stdin_handling() {
    // true should not read from stdin but should still succeed
    mtrue_cmd()
        .write_stdin("some input\n")
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_very_long_argument() {
    // true should handle very long arguments
    let long_arg = "a".repeat(10000);
    mtrue_cmd()
        .arg(&long_arg)
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_empty_arguments() {
    // true should handle empty string arguments
    mtrue_cmd()
        .args(&["", "", ""])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}

#[test]
fn test_dash_arguments() {
    // true should ignore dash arguments that might be interpreted as flags
    mtrue_cmd()
        .args(&["-", "--", "---", "-abc"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
}