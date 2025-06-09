use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mwc"));
}

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("nonexistent_file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn test_default_behavior_single_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Hello world\nThis is a test file\nWith multiple lines\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+8\s+40\s+.*test\.txt$").unwrap());
}

#[test]
fn test_lines_only() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Line 1\nLine 2\nLine 3\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-l", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+.*test\.txt$").unwrap());
}

#[test]
fn test_words_only() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("one two three four five").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-w", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*5\s+.*test\.txt$").unwrap());
}

#[test]
fn test_bytes_only() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Hello").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-c", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*5\s+.*test\.txt$").unwrap());
}

#[test]
fn test_characters_only() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Hello 世界").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-m", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*8\s+.*test\.txt$").unwrap());
}

#[test]
fn test_max_line_length() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("short\nthis is a much longer line\nmedium line").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-L", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*26\s+.*test\.txt$").unwrap());
}

#[test]
fn test_multiple_files() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file1 = temp.child("test1.txt");
    let test_file2 = temp.child("test2.txt");
    
    test_file1.write_str("File 1\nContent\n").unwrap();
    test_file2.write_str("File 2\nDifferent content\nWith more lines\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        test_file1.path().to_str().unwrap(),
        test_file2.path().to_str().unwrap()
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("total"));
}

#[test]
fn test_stdin_input() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("-")
        .write_stdin("Hello world\nFrom stdin\n")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*2\s+4\s+25$").unwrap());
}

#[test]
fn test_empty_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("empty.txt");
    test_file.write_str("").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*0\s+0\s+0\s+.*empty\.txt$").unwrap());
}

#[test]
fn test_combined_flags() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Hello world\nSecond line\n").unwrap();

    // Test -l -w combination
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-l", "-w", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*2\s+4\s+.*test\.txt$").unwrap());
}

#[test]
fn test_long_flags() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Hello world\n").unwrap();

    // Test --lines
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--lines", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*1\s+.*test\.txt$").unwrap());

    // Test --words
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--words", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*2\s+.*test\.txt$").unwrap());

    // Test --bytes
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--bytes", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*12\s+.*test\.txt$").unwrap());

    // Test --chars
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--chars", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*12\s+.*test\.txt$").unwrap());

    // Test --max-line-length
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--max-line-length", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*11\s+.*test\.txt$").unwrap());
}

#[test]
fn test_whitespace_handling() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("whitespace.txt");
    test_file.write_str("  word1   word2  \n\n  word3\t\tword4  \n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+4\s+\d+\s+.*whitespace\.txt$").unwrap());
}

#[test]
fn test_utf8_characters() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("utf8.txt");
    test_file.write_str("Hello 世界 🌍\n").unwrap();

    // Test character count vs byte count
    let mut cmd_chars = Command::cargo_bin("mwc").unwrap();
    cmd_chars.args(&["-m", test_file.path().to_str().unwrap()])
        .assert()
        .success();

    let mut cmd_bytes = Command::cargo_bin("mwc").unwrap();
    cmd_bytes.args(&["-c", test_file.path().to_str().unwrap()])
        .assert()
        .success();
}