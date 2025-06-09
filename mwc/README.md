# mwc

A Rust reimplementation of the `wc` (word count) utility.

## About

`mwc` is a behavioral clone of the standard Unix `wc` command, designed as a learning project to practice Rust programming. It aims to match the behavior and output of the original `wc` utility as closely as possible.

## Features

- Count lines, words, characters, and bytes in files
- Support for multiple files with totals
- Read from standard input when no files specified
- Unicode-aware character counting
- Maximum line length calculation
- Compatible command-line interface with `wc`

## Usage

```bash
# Count lines, words, and bytes (default behavior)
mwc file.txt

# Count only lines
mwc -l file.txt

# Count only words  
mwc -w file.txt

# Count only bytes
mwc -c file.txt

# Count only characters (Unicode-aware)
mwc -m file.txt

# Show maximum line length
mwc -L file.txt

# Process multiple files
mwc file1.txt file2.txt

# Read from standard input
echo "hello world" | mwc

# Use long options
mwc --lines --words file.txt
```

## Options

- `-l, --lines` - Print newline counts
- `-w, --words` - Print word counts  
- `-c, --bytes` - Print byte counts
- `-m, --chars` - Print character counts
- `-L, --max-line-length` - Print maximum line length
- `--help` - Display help information
- `--version` - Display version information

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

The test suite includes comprehensive integration tests that verify `mwc` behavior matches the standard `wc` utility.

## Implementation Goals

This project focuses on:
- Exact behavioral compatibility with `wc`
- Proper handling of Unicode text
- Efficient file processing
- Comprehensive error handling
- Clean, idiomatic Rust code

## License

MIT OR Apache-2.0