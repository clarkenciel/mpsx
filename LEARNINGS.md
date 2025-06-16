# LEARNINGS.md

## `wc` Behavioral Insights

This document captures insights about GNU `wc` behavior discovered during the implementation of `mwc`.

### Stdin Handling

#### Multiple `-` Arguments
When multiple `-` arguments are provided, `wc` exhibits quirky behavior:

```bash
echo 'hi' | wc - - - -
# Output:
#       1       1       3 -
#       0       0       0 -  
#       0       0       0 -
#       0       0       0 -
#       1       1       3 total
```

**Explanation**: 
- Only the first `-` receives the piped input
- Subsequent `-` arguments get empty input (0 counts) because stdin is exhausted
- This is a consequence of Unix pipe behavior - stdin can only be read once per process
- Each `-` is treated as a separate "file" that attempts to read from stdin

#### Stdin Mixed with Files
`wc` processes arguments in order, allowing stdin to be mixed with regular files:

```bash
echo 'hi' | wc - Cargo.toml
echo 'hi' | wc Cargo.toml -
```

Both work correctly, with stdin processed at the position of the `-` argument.

#### Default Stdin Behavior
When no arguments are provided, `wc` defaults to reading from stdin:

```bash
wc  # Waits for stdin input
```

This behavior is documented in the manual: "With no FILE, or when FILE is -, read standard input."

### Output Formatting

#### Column Width Inconsistency
`wc` has inconsistent column spacing behavior that is **not documented** in the manual:

**Files only** (tight spacing):
```bash
wc file1 file2
# Output: " 21  50 429 file1"
```

**With stdin** (wide spacing):
```bash
echo 'hi' | wc file1 -
# Output: "     21      50     429 file1"
```

**Key insights**:
- The spacing change is triggered by the presence of any `-` argument
- It's not related to filename length
- This appears to be an undocumented implementation detail
- The wide spacing uses multiple spaces, not tabs

#### Filename Display
- Regular files show their full path/name
- Stdin is always displayed as `-`
- The manual doesn't specify formatting details beyond the order of counts

### Implementation Decisions for `mwc`

Based on these findings, `mwc` makes the following choices:

1. **Consistent formatting**: Use uniform column spacing regardless of stdin presence (simpler and more predictable)
2. **Match functional behavior**: Implement the same stdin handling logic, including the "multiple `-`" quirk
3. **Document clearly**: Explicit help text about stdin behavior since it's not obvious

### Testing Implications

These behaviors should be covered in integration tests:

- Multiple `-` arguments (first gets data, rest get zeros)
- Mixing stdin with files in different orders
- Default stdin when no arguments provided
- Consistent output formatting across scenarios

### Manual Page Limitations

The `wc` manual page doesn't document:
- Output formatting details (spacing, alignment)
- The multiple `-` behavior quirk
- Column width behavior differences

This gives implementers flexibility in formatting choices while maintaining functional compatibility.

## Character Counting Behavior (`-m` flag)

### UTF-8 vs Locale-Aware Counting

`wc -m` uses **locale-aware character counting** via the `LC_CTYPE` environment variable and `mbrtowc(3)` function, not just UTF-8:

```bash
printf '\xe9' | LC_CTYPE=ISO-8859-1 wc -m  # 1 character (é in ISO-8859-1)
printf '\xe9' | LC_CTYPE=C.UTF-8 wc -m     # 0 characters (invalid UTF-8)
```

### Invalid UTF-8 Handling

`wc -m` **drops invalid UTF-8 sequences entirely**:

```bash
printf '\xff\xfe' | wc -m  # 0 characters
printf '\xff\xfe' | wc -c  # 2 bytes
```

### Rust Implementation Differences

Rust's `str::chars()` approach:
- **Always assumes UTF-8** regardless of locale
- Works for 99% of modern systems (UTF-8 default)
- **Not fully compatible** with `wc`'s locale-aware behavior
- Simpler implementation but sacrifices edge case compatibility

### Character vs Byte Counting

- `wc -c`: Counts raw bytes (encoding-agnostic)
- `wc -m`: Counts Unicode scalar values (locale-dependent character decoding)
- Both handle multibyte characters differently:
  - "café" = 5 bytes, 4 characters
  - "🚀" = 4 bytes, 1 character

### Implementation Trade-offs

For educational POSIX utilities:
- **Full compatibility**: Requires complex locale handling with multiple encoding support
- **Practical approach**: UTF-8-only covers most real-world usage
- **Real `wc`** uses system locale libraries; Rust uses built-in UTF-8 validation

This represents a fundamental difference between system utilities (locale-aware) and modern language implementations (UTF-8-centric).

## Max Line Length Behavior (`-L` flag)

### Locale-Aware Display Width Calculation

`wc -L` is **fully locale-aware** and measures **display width**, not just character count or byte count.

**Evidence of locale dependency:**
```bash
printf '中文字符' | LC_CTYPE=C.UTF-8 wc -L     # → 8 (wide chars = 2 columns each)
printf '中文字符' | LC_CTYPE=ISO-8859-1 wc -L # → 0 (can't decode UTF-8)

printf 'café' | LC_CTYPE=C.UTF-8 wc -L        # → 4 (proper UTF-8 decoding)
printf 'café' | LC_CTYPE=ISO-8859-1 wc -L     # → 3 (UTF-8 'é' bytes invalid)
```

### Invalid Sequence Handling

**Key behavior:** `wc -L` returns **0** when encountering character sequences that can't be decoded in the current locale, rather than attempting to guess display width.

**Pattern:**
- **Valid sequences**: Calculate display width based on character properties
- **Invalid sequences**: Return 0 (fail-safe approach)
- **Wide characters**: Count as 2 display columns (e.g., Chinese characters)

### Implementation Complexity

`wc -L` is the **most complex** `wc` feature because it requires:

1. **Locale detection** (`LC_CTYPE` environment variable)
2. **Character decoding** (multibyte sequence handling per locale)
3. **Display width calculation** (Unicode character width properties)
4. **Invalid sequence handling** (graceful failure to 0)

### Comparison with Other Flags

- `wc -c`: Raw bytes (encoding-agnostic)
- `wc -m`: Characters (locale-aware decoding)  
- `wc -L`: Display width (locale-aware decoding + width calculation)

**Character vs Display Width:**
```bash
printf 'éééé' | wc -m  # → 4 characters
printf 'éééé' | wc -c  # → 8 bytes  
printf 'éééé' | wc -L  # → 4 display width

printf '中文字符' | wc -m  # → 4 characters
printf '中文字符' | wc -c  # → 12 bytes
printf '中文字符' | wc -L  # → 8 display width (wide chars)
```

### Rust Implementation Challenges

For full `wc -L` compatibility, a Rust implementation needs:
- **Locale detection**: Read `LC_CTYPE` environment variable
- **Encoding libraries**: `encoding_rs` for multibyte character decoding
- **Display width**: `unicode-width` crate for character width calculation
- **Error handling**: Return 0 for invalid sequences

This makes `-L` significantly more complex than the other counting modes.