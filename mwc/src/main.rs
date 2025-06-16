use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    ops::AddAssign,
    path::{Path, PathBuf},
};

use clap::Parser;
use unicode_width::UnicodeWidthStr;

fn main() -> std::io::Result<()> {
    let mut opts = Opts::parse();
    let inputs = if let Some(ref inputs_input) = opts.files_from {
        let mut reader = inputs_input
            .reader()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut inputs = Vec::new();
        let mut buf = Vec::new();
        while let Ok(bytes_read) = reader.read_until(0x0, &mut buf) {
            if bytes_read == 0 {
                break;
            }
            let input_str = std::str::from_utf8(&buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
                .trim_end_matches('\0');
            let input = parse_input(input_str)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            inputs.push(input);
            buf.clear();
        }
        inputs
    } else {
        std::mem::take(&mut opts.inputs)
    };

    let mut counter = Counter::new();
    let input_count = inputs.len();
    let display = (&opts).into();

    // If you provide no inputs wc will try to read from stdin.
    // You can type whatever and then hit Ctrl-D to get the wc stats for what you typed.
    if input_count == 0 {
        counter.count_stdin();
    } else {
        for input in inputs {
            counter.count_input(input);
        }
    }

    let mut output = Printer {
        display,
        widths: counter.widths.clone(),
    };
    let mut any_errors = false;
    let mut stdout = IoToFmt::stdout();
    for result in counter.file_counts {
        match result {
            Ok(CountedInput(CountsName::StdIn, file_counts)) if input_count < 2 => {
                output
                    .print_result(&mut stdout, CountsName::Blank, file_counts)
                    .expect("TODO: HANDLE ME");
            }
            Ok(CountedInput(file, file_count)) => {
                output
                    .print_result(&mut stdout, file, file_count)
                    .expect("TODO: HANDLE ME");
            }
            Err(error) => {
                eprintln!("{}", error);
                any_errors = true;
            }
        }
    }

    if input_count > 1 {
        output.widths = counter.widths.max((&counter.totals).into());
        output
            .print_result(&mut stdout, CountsName::Total, counter.totals)
            .expect("TODO: HANDLE ME");
    }

    if any_errors {
        std::process::exit(1)
    } else {
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("mwc: {0}: No such file or directory")]
    NoFile(PathBuf),
    #[error("mwc: {0}: Is a directory")]
    IsDirectory(PathBuf),
    #[error("mwc: {0}: File read error: {1}")]
    FileCount(PathBuf, std::io::Error),
}

struct CountedInput(CountsName, FileCounts);

struct Counter {
    file_counts: Vec<Result<CountedInput, Error>>,
    widths: ColumnWidths,
    totals: FileCounts,
    stdin_seen: bool,
}

impl Counter {
    fn new() -> Self {
        Self {
            file_counts: Vec::new(),
            widths: ColumnWidths::default(),
            totals: FileCounts::default(),
            stdin_seen: false,
        }
    }

    fn count_input(&mut self, input: Input) {
        match input {
            Input::File(path_buf) => self.count_file(path_buf),
            Input::StdIn if !self.stdin_seen => {
                self.count_stdin();
                self.stdin_seen = true;
            }
            Input::StdIn => self.count_default(),
        }
    }

    fn count_file(&mut self, pb: PathBuf) {
        if !pb.exists() {
            self.file_counts.push(Err(Error::NoFile(pb)));
            return;
        }

        if pb.is_dir() {
            self.file_counts.push(Err(Error::IsDirectory(pb)));
            return;
        }

        match std::fs::File::open(&pb) {
            Ok(f) => self.count_reader(CountsName::File(pb), BufReader::new(f)),
            Err(e) => self.file_counts.push(Err(Error::FileCount(pb, e))),
        }
    }

    fn count_stdin(&mut self) {
        self.count_reader(CountsName::StdIn, BufReader::new(std::io::stdin().lock()));
    }

    fn count_reader(&mut self, name: CountsName, content: impl BufRead) {
        let counts = FileCounts::from_reader(content);

        self.apply_counts(name, counts);
    }

    fn apply_counts(&mut self, name: CountsName, counts: FileCounts) {
        self.widths = self.widths.max((&counts).into());
        self.totals += &counts;
        self.file_counts.push(Ok(CountedInput(name, counts)));
    }

    fn count_default(&mut self) {
        self.apply_counts(CountsName::StdIn, FileCounts::default());
    }
}

struct CountsConfig {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}

impl Default for CountsConfig {
    fn default() -> Self {
        Self {
            lines: true,
            words: true,
            bytes: true,
            chars: false,
            max_line_length: false,
        }
    }
}

impl CountsConfig {
    fn new() -> Self {
        Self {
            lines: false,
            words: false,
            bytes: false,
            chars: false,
            max_line_length: false,
        }
    }
}

impl From<&Opts> for CountsConfig {
    fn from(opts: &Opts) -> Self {
        // only set specific flags if _some_ option was passed
        if opts.lines || opts.bytes || opts.chars || opts.words || opts.max_line_length {
            let mut out = Self::new();
            out.lines = out.lines || opts.lines;
            out.bytes = out.bytes || opts.bytes;
            out.chars = out.chars || opts.chars;
            out.words = out.words || opts.words;
            out.max_line_length = out.max_line_length || opts.max_line_length;
            out
        } else {
            Self::default()
        }
    }
}

enum CountsName {
    File(PathBuf),
    StdIn,
    Blank,
    Total,
}

impl Display for CountsName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountsName::File(path_buf) => write!(f, " {}", path_buf.display()),
            CountsName::StdIn => write!(f, " -"),
            CountsName::Blank => write!(f, ""),
            CountsName::Total => write!(f, " total"),
        }
    }
}

#[derive(Parser)]
#[command(version)]
struct Opts {
    #[arg(
        value_parser=parse_input,
        help="Input files (use `-` for stdin, default: read from stdin if no files provided)"
    )]
    inputs: Vec<Input>,

    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
    #[arg(short = 'c', long)]
    bytes: bool,
    #[arg(short = 'm', long)]
    chars: bool,
    #[arg(short = 'L', long)]
    max_line_length: bool,

    #[arg(long = "files0-from", value_parser = parse_input)]
    files_from: Option<Input>,
}

#[derive(Clone)]
enum Input {
    File(PathBuf),
    StdIn,
}

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

impl Input {
    fn reader(&self) -> Result<InputBufReader, BoxedError> {
        match self {
            Input::File(path_buf) => Ok(BufReader::new(InputReader::file(path_buf)?)),
            Input::StdIn => Ok(BufReader::new(InputReader::stdin())),
        }
    }
}

type InputBufReader = BufReader<InputReader>;

enum InputReader {
    StdIn(std::io::StdinLock<'static>),
    File(std::fs::File),
}

impl InputReader {
    fn stdin() -> Self {
        Self::StdIn(std::io::stdin().lock())
    }

    fn file<P: AsRef<Path>>(pb: P) -> Result<Self, BoxedError> {
        Ok(Self::File(std::fs::File::open(pb.as_ref())?))
    }
}

impl std::io::Read for InputReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            InputReader::StdIn(stdin_lock) => stdin_lock.read(buf),
            InputReader::File(file) => file.read(buf),
        }
    }
}

fn parse_input(s: &str) -> Result<Input, clap::Error> {
    if s == "-" {
        Ok(Input::StdIn)
    } else {
        Ok(Input::File(s.into()))
    }
}

#[derive(Default, Debug)]
struct FileCounts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

const NEWLINE: u8 = 0x0a;

impl FileCounts {
    // wc behavior: Always returns partial counts collected before stream ends.
    // Does not report errors for EOF/broken pipes - only outputs whatever
    // was successfully counted. File-level errors (permissions, not found)
    // are handled separately at the file opening stage.
    fn from_reader(mut reader: impl BufRead) -> Self {
        let mut counts = Self::default();
        let mut buf = Vec::new();
        while let Ok(bytes_read) = reader.read_until(NEWLINE, &mut buf) {
            if bytes_read == 0 {
                break;
            }
            counts.bytes += bytes_read;
            counts.lines += 1;
            counts.words += buf
                .split(u8::is_ascii_whitespace)
                .filter(|s| !s.is_empty())
                .count();
            let s = std::str::from_utf8(&buf);
            counts.chars += s.map(|s| s.chars().count()).unwrap_or(0);
            counts.max_line_length = counts
                .max_line_length
                .max(s.map(|s| s.trim_end_matches('\n').width()).unwrap_or(0));
            buf.clear();
        }

        counts
    }
}

impl AddAssign<&Self> for FileCounts {
    fn add_assign(&mut self, rhs: &Self) {
        self.lines += rhs.lines;
        self.bytes += rhs.bytes;
        self.words += rhs.words;
        self.chars += rhs.chars;
        self.max_line_length = self.max_line_length.max(rhs.max_line_length);
    }
}

#[derive(Debug, Clone, Copy)]
struct ColumnWidths {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

impl ColumnWidths {
    fn max(&self, other: Self) -> Self {
        Self {
            lines: self.lines.max(other.lines),
            words: self.words.max(other.words),
            bytes: self.bytes.max(other.bytes),
            chars: self.chars.max(other.chars),
            max_line_length: self.max_line_length.max(other.max_line_length),
        }
    }
}

impl Default for ColumnWidths {
    fn default() -> Self {
        Self {
            lines: 2,
            words: 2,
            bytes: 2,
            chars: 2,
            max_line_length: 2,
        }
    }
}

impl From<&FileCounts> for ColumnWidths {
    fn from(value: &FileCounts) -> Self {
        Self {
            lines: 2.max(digits(value.lines) + 1),
            words: 2.max(digits(value.words) + 1),
            bytes: 2.max(digits(value.bytes) + 1),
            chars: 2.max(digits(value.chars) + 1),
            max_line_length: 2.max(digits(value.max_line_length) + 1),
        }
    }
}

// Find the number of digits in a usize
fn digits(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut digs = 0;
    let mut n = n;
    while n > 0 {
        n /= 10;
        digs += 1;
    }

    digs
}

#[test]
fn test_digits() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(9), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(99), 2);
    assert_eq!(digits(100), 3);
    assert_eq!(digits(101), 3);
}

struct Printer {
    widths: ColumnWidths,
    display: CountsConfig,
}

impl Printer {
    fn print_counts<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        counts: &FileCounts,
    ) -> Result<(), BoxedError> {
        if self.display.lines {
            write!(
                writer,
                "{:>width$}",
                counts.lines,
                width = self.widths.lines
            )?;
        }
        if self.display.words {
            write!(
                writer,
                "{:>width$}",
                counts.words,
                width = self.widths.words
            )?;
        }
        if self.display.bytes {
            write!(
                writer,
                "{:>width$}",
                counts.bytes,
                width = self.widths.bytes
            )?;
        }
        if self.display.chars {
            write!(
                writer,
                "{:>width$}",
                counts.chars,
                width = self.widths.chars
            )?;
        }
        if self.display.max_line_length {
            write!(
                writer,
                "{:>width$}",
                counts.max_line_length,
                width = self.widths.max_line_length
            )?;
        }

        Ok(())
    }

    fn print_file<W: std::fmt::Write, D: Display>(
        &self,
        writer: &mut W,
        file: D,
    ) -> Result<(), BoxedError> {
        write!(writer, "{}", file).map_err(Into::into)
    }

    fn print_result<W: std::fmt::Write, D: Display>(
        &self,
        writer: &mut W,
        file: D,
        file_count: FileCounts,
    ) -> Result<(), BoxedError> {
        self.print_counts(writer, &file_count)?;
        self.print_file(writer, &file)?;
        writeln!(writer).map_err(Into::into)
    }
}

struct IoToFmt<W: std::io::Write>(W);

impl IoToFmt<std::io::StdoutLock<'_>> {
    fn stdout() -> Self {
        Self(std::io::stdout().lock())
    }
}

impl<W: std::io::Write> std::fmt::Write for IoToFmt<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}

#[test]
fn test_output_format() {
    let mut output = String::new();
    Printer {
        display: CountsConfig::default(),
        widths: ColumnWidths::default(),
    }
    .print_counts(
        &mut output,
        &FileCounts {
            lines: 1,
            words: 1,
            bytes: 6,
            chars: 2,
            max_line_length: 1,
        },
    )
    .unwrap();
    assert_eq!(" 1 1 6", output);
}
