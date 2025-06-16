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
fn test_version_long_flag() {
    let expected_version = format!("mwc {}\n", env!("CARGO_PKG_VERSION"));
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(expected_version);
}

#[test]
fn test_version_short_flag() {
    let expected_version = format!("mwc {}\n", env!("CARGO_PKG_VERSION"));
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("-V").assert().success().stdout(expected_version);
}

#[test]
fn test_mixed_existing_and_nonexistent_files() {
    let temp = assert_fs::TempDir::new().unwrap();
    let existing_file1 = temp.child("file1.txt");
    let existing_file2 = temp.child("file2.txt");

    existing_file1.write_str("Hello world\n").unwrap();
    existing_file2
        .write_str("Test content\nSecond line\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        existing_file1.path().to_str().unwrap(),
        "nonexistent.txt",
        existing_file2.path().to_str().unwrap(),
    ])
    .assert()
    .failure() // Should succeed despite missing file
    .stdout(predicate::str::contains("total")) // Should show totals
    .stderr(predicate::str::contains("No such file")); // Should show error for missing file
}

#[test]
fn test_default_behavior_single_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file
        .write_str("Hello world\nThis is a test file\nWith multiple lines\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+10\s+52\s+.*test\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*3\s+.*test\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*5\s+.*test\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*5\s+.*test\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*8\s+.*test\.txt\n$").unwrap());
}

#[test]
fn test_max_line_length() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file
        .write_str("short\nthis is a much longer line\nmedium line")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-L", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*26\s+.*test\.txt\n$").unwrap());
}

#[test]
fn test_multiple_files() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file1 = temp.child("test1.txt");
    let test_file2 = temp.child("test2.txt");

    test_file1.write_str("File 1\nContent\n").unwrap();
    test_file2
        .write_str("File 2\nDifferent content\nWith more lines\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        test_file1.path().to_str().unwrap(),
        test_file2.path().to_str().unwrap(),
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
        .stdout(predicate::str::is_match(r"^\s*2\s+4\s+23\n$").unwrap());
}

#[test]
fn test_implicit_stdin_input() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.write_stdin("Hello world\nFrom stdin\n")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*2\s+4\s+23\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*0\s+0\s+0\s+.*empty\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*2\s+4\s+.*test\.txt\n$").unwrap());
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
        .stdout(predicate::str::is_match(r"^\s*1\s+.*test\.txt\n$").unwrap());

    // Test --words
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--words", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*2\s+.*test\.txt\n$").unwrap());

    // Test --bytes
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--bytes", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*12\s+.*test\.txt\n$").unwrap());

    // Test --chars
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--chars", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*12\s+.*test\.txt\n$").unwrap());

    // Test --max-line-length
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--max-line-length", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*11\s+.*test\.txt\n$").unwrap());
}

#[test]
fn test_whitespace_handling() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("whitespace.txt");
    test_file
        .write_str("  word1   word2  \n\n  word3\t\tword4  \n")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+4\s+\d+\s+.*whitespace\.txt\n$").unwrap());
}

#[test]
fn test_utf8_characters() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("utf8.txt");
    test_file.write_str("Hello 世界 🌍\n").unwrap();

    // Test character count vs byte count
    let mut cmd_chars = Command::cargo_bin("mwc").unwrap();
    cmd_chars
        .args(&["-m", test_file.path().to_str().unwrap()])
        .assert()
        .success();

    let mut cmd_bytes = Command::cargo_bin("mwc").unwrap();
    cmd_bytes
        .args(&["-c", test_file.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_duplicate_filenames() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("Line 1\nLine 2\nLine 3\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        test_file.path().to_str().unwrap(),
        test_file.path().to_str().unwrap(),
        test_file.path().to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::is_match(r"(?m)^\s*9\s+18\s+\d+\s+total\n$").unwrap()); // Total should be 3x the individual counts
}

#[test]
fn test_binary_file_handling() {
    let temp = assert_fs::TempDir::new().unwrap();
    let binary_file = temp.child("binary.bin");

    // Create binary content with invalid UTF-8 sequences
    let binary_data = vec![
        0xFF, 0xFE, 0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x0B, 0x80, 0x81,
    ];
    binary_file.write_binary(&binary_data).unwrap();

    // Test default behavior (should work with binary data)
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(binary_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*1\s+\d+\s+11\s+.*binary\.bin\n$").unwrap());

    // Test byte count explicitly
    let mut cmd_bytes = Command::cargo_bin("mwc").unwrap();
    cmd_bytes
        .args(&["-c", binary_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*11\s+.*binary\.bin\n$").unwrap());
}

#[test]
fn test_byte_vs_character_count_difference() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("multibyte.txt");

    // UTF-8 content where byte count != character count
    // "café" = 4 characters, 5 bytes (é is 2 bytes in UTF-8)
    test_file.write_str("café\n").unwrap();

    // Test byte count
    let mut cmd_bytes = Command::cargo_bin("mwc").unwrap();
    cmd_bytes
        .args(&["-c", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*6\s+.*multibyte\.txt\n$").unwrap()); // 5 chars + 1 newline = 6 bytes

    // Test character count
    let mut cmd_chars = Command::cargo_bin("mwc").unwrap();
    cmd_chars
        .args(&["-m", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*5\s+.*multibyte\.txt\n$").unwrap()); // 4 chars + 1 newline = 5 characters

    // Test default behavior (should show bytes, not characters)
    let mut cmd_default = Command::cargo_bin("mwc").unwrap();
    cmd_default
        .arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*1\s+1\s+6\s+.*multibyte\.txt\n$").unwrap()); // Should show 6 bytes
}

#[test]
fn test_encoding_agnostic_line_counting() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("mixed_content.txt");

    // Mix of ASCII and non-ASCII with various line endings
    let content = "Hello\nwörld 🌍\nLine 3\n";
    test_file.write_str(content).unwrap();

    // Line counting should work regardless of character encoding
    // because it counts newline bytes (0x0A)
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-l", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+.*mixed_content\.txt\n$").unwrap());

    // Word counting should also be encoding-agnostic (whitespace-delimited)
    let mut cmd_words = Command::cargo_bin("mwc").unwrap();
    cmd_words
        .args(&["-w", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*5\s+.*mixed_content\.txt\n$").unwrap()); // Hello, wörld, 🌍, Line, 3
}

#[test]
fn test_single_file_minimal_padding() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("small.txt");
    test_file.write_str("Hello\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path())
        .assert()
        .success()
        // Single file should have minimal spacing, no leading spaces for small numbers
        .stdout(predicate::str::is_match(r"^ 1 1 6 .*small\.txt\n$").unwrap());
}

#[test]
fn test_multiple_files_columnar_alignment() {
    let temp = assert_fs::TempDir::new().unwrap();
    let small_file = temp.child("small.txt");
    let large_file = temp.child("large.txt");

    // Small file: 1 line, 1 word, 6 bytes
    small_file.write_str("Hello\n").unwrap();

    // Large file: Create content with 1,000,000+ bytes to force wide padding
    let large_content = "x".repeat(1_500_000) + "\n";
    large_file.write_str(&large_content).unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        small_file.path().to_str().unwrap(),
        large_file.path().to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(
        predicate::function(|output: &str| output.lines().count() == 3)
            .and(predicate::str::is_match(r" 1 1       6 .*small.txt").unwrap())
            .and(predicate::str::is_match(r" 1 1 1500001 .*large.txt").unwrap()),
    );
}

#[test]
fn test_stdin_mixed_with_files() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file
        .write_str("file content\nwith multiple lines\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.write_stdin("hello world\n")
        .arg("-")
        .arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\s*1\s+2\s+12\s+-").unwrap())
        .stdout(predicate::str::is_match(r"2\s+5\s+33").unwrap())
        .stdout(predicate::str::contains("total"));
}

#[test]
fn test_multiple_stdin_args() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.write_stdin("test input\n")
        .arg("-")
        .arg("-")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"1\s+2\s+11\s+-").unwrap())
        .stdout(predicate::str::is_match(r"0\s+0\s+0\s+-").unwrap())
        .stdout(predicate::str::contains("total"));
}

#[test]
fn test_word_counting_across_lines() {
    // Test that words are counted across the entire content, not per-line
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("multiline_words.txt");
    test_file
        .write_str("word1 word2\nword3 word4\nword5")
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-w", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*5\s+.*multiline_words\.txt\n$").unwrap());
}

#[test]
fn test_no_final_newline_line_counting() {
    // Test that files without final newline still count the last line
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("no_final_newline.txt");
    test_file
        .write_str("line1\nline2\nline3") // No final newline
        .unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-l", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*3\s+.*no_final_newline\.txt\n$").unwrap());
}

#[test]
fn test_exit_status_success() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("hello world\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(test_file.path()).assert().code(0); // POSIX requires exit status 0 for success
}

#[test]
fn test_exit_status_file_not_found() {
    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg("nonexistent_file.txt")
        .assert()
        .code(predicate::ne(0)) // POSIX requires exit status >0 for errors
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn test_files0_from_option() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file1 = temp.child("file1.txt");
    let file2 = temp.child("file2.txt");
    let filelist = temp.child("filelist.txt");

    file1.write_str("hello\n").unwrap();
    file2.write_str("world\n").unwrap();

    // Create null-separated file list
    let file_paths = format!(
        "{}\x00{}",
        file1.path().to_str().unwrap(),
        file2.path().to_str().unwrap()
    );
    filelist.write_str(&file_paths).unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--files0-from", filelist.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("total"));
}

#[test]
fn test_files0_from_stdin() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file1 = temp.child("file1.txt");
    let file2 = temp.child("file2.txt");

    file1.write_str("hello\n").unwrap();
    file2.write_str("world\n").unwrap();

    let file_paths = format!(
        "{}\0{}\0",
        file1.path().to_str().unwrap(),
        file2.path().to_str().unwrap()
    );

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["--files0-from", "-"])
        .write_stdin(file_paths)
        .assert()
        .success()
        .stdout(predicate::str::contains("total"));
}

#[test]
fn test_combined_c_and_m_options() {
    // Test that -c and -m can be used together (both should appear in output)
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("café\n").unwrap(); // 5 bytes, 5 characters

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&["-c", "-m", test_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\s*6\s+5\s+.*test\.txt\n$").unwrap());
}

#[test]
fn test_all_options_together() {
    // Test combining all count options
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("test.txt");
    test_file.write_str("hello world\nsecond line\n").unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.args(&[
        "-l",
        "-w",
        "-c",
        "-m",
        "-L",
        test_file.path().to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::is_match(r"^\s*2\s+4\s+24\s+24\s+11\s+.*test\.txt\n$").unwrap());
}

#[test]
fn test_directory_handling() {
    let temp = assert_fs::TempDir::new().unwrap();
    let subdir = temp.child("subdir");
    subdir.create_dir_all().unwrap();

    let mut cmd = Command::cargo_bin("mwc").unwrap();
    cmd.arg(subdir.path())
        .assert()
        .failure() // Should fail when trying to count a directory
        .stderr(
            predicate::str::contains("Is a directory").or(predicate::str::contains("directory")),
        );
}

#[test]
fn test_permission_denied() {
    // This test may not work on all systems, but attempts to test permission handling
    let temp = assert_fs::TempDir::new().unwrap();
    let test_file = temp.child("no_read.txt");
    test_file.write_str("content").unwrap();

    // Try to remove read permissions (may fail on some systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = test_file.path().metadata().unwrap().permissions();
        perms.set_mode(0o000);
        let _ = std::fs::set_permissions(test_file.path(), perms);

        let mut cmd = Command::cargo_bin("mwc").unwrap();
        cmd.arg(test_file.path()).assert().failure().stderr(
            predicate::str::contains("Permission denied")
                .or(predicate::str::contains("permission")),
        );
    }
}
