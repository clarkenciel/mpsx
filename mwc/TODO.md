# mwc TODO

## Locale-Aware Character Counting (`-m` flag)

### Current Status
The current implementation uses Rust's `str::chars()` which assumes UTF-8 encoding. This works for 99% of modern systems but is not fully compatible with GNU `wc`'s locale-aware behavior.

### Problem
GNU `wc -m` uses:
- `LC_CTYPE` environment variable for encoding detection
- `mbrtowc(3)` system function for multibyte character decoding
- Supports multiple encodings (UTF-8, ISO-8859-1, etc.)

**Example difference:**
```bash
printf '\xe9' | LC_CTYPE=ISO-8859-1 wc -m  # 1 character (é in ISO-8859-1)
printf '\xe9' | LC_CTYPE=C.UTF-8 wc -m     # 0 characters (invalid UTF-8)
```

Rust always treats this as invalid UTF-8 and returns 0.

### Implementation Options

#### Option 1: Simple UTF-8 (Current)
**Pros:** Simple, works for most cases
**Cons:** Not fully wc-compatible

```rust
counts.chars += std::str::from_utf8(&buf)
    .map(|s| s.chars().count())
    .unwrap_or(0);
```

#### Option 2: Full Locale Support
**Pros:** Complete wc compatibility
**Cons:** Complex implementation, additional dependencies

**Dependencies needed:**
```bash
cargo add encoding_rs
cargo add encoding_rs_io
```

**Implementation:**
```rust
use std::env;
use encoding_rs::*;

fn get_locale_encoding() -> &'static str {
    // Follow POSIX locale precedence: LC_ALL > LC_CTYPE > LANG
    let locale = env::var("LC_ALL")
        .or_else(|_| env::var("LC_CTYPE"))
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C".to_string());
    
    // Parse encoding from locale (e.g., "en_US.UTF-8" -> "UTF-8")
    if let Some(encoding) = locale.split('.').nth(1) {
        encoding
    } else if locale.starts_with("C") {
        "ASCII"
    } else {
        "UTF-8" // Default fallback
    }
}

fn count_characters_locale_aware(bytes: &[u8]) -> usize {
    let encoding_name = get_locale_encoding();
    
    match encoding_name {
        "UTF-8" => {
            std::str::from_utf8(bytes)
                .map(|s| s.chars().count())
                .unwrap_or(0) // Drop invalid sequences like wc
        }
        "ISO-8859-1" | "LATIN1" => {
            // Each byte is a character in Latin-1
            bytes.len()
        }
        _ => {
            // Use encoding_rs for other encodings
            if let Some(encoding) = Encoding::for_label(encoding_name.as_bytes()) {
                let (decoded, _, had_errors) = encoding.decode(bytes);
                if had_errors {
                    0 // Drop invalid sequences
                } else {
                    decoded.chars().count()
                }
            } else {
                // Fallback to UTF-8 for unknown encodings
                std::str::from_utf8(bytes)
                    .map(|s| s.chars().count())
                    .unwrap_or(0)
            }
        }
    }
}
```

**Usage in `FileCounts::from_reader`:**
```rust
counts.chars += count_characters_locale_aware(&buf);
```

#### Option 3: Hybrid Approach
**Pros:** Covers most real-world cases with moderate complexity
**Cons:** Still not 100% compatible

Support UTF-8 + ISO-8859-1 only:
```rust
fn count_characters_hybrid(bytes: &[u8]) -> usize {
    let locale = std::env::var("LC_CTYPE").unwrap_or_default();
    
    if locale.contains("ISO-8859-1") || locale.contains("LATIN1") {
        bytes.len() // Each byte is a character in Latin-1
    } else {
        std::str::from_utf8(bytes)
            .map(|s| s.chars().count())
            .unwrap_or(0)
    }
}
```

### Recommendation

For an **educational POSIX utility reimplementation**:
- **Current approach (Option 1)** is acceptable for learning purposes
- **Document the limitation** in README/comments
- **Option 2** if you want full compatibility and are willing to handle the complexity
- **Option 3** as a middle ground

### Current Bug Fix Needed

**Issue:** Line 262 in `main.rs` uses assignment (`=`) instead of addition (`+=`):
```rust
// BUG: Only counts last buffer
counts.chars = std::str::from_utf8(&buf)...

// FIX: Accumulate counts
counts.chars += std::str::from_utf8(&buf)...
```

### Testing Requirements

Add integration tests for:
- UTF-8 multibyte characters: `"café"` should count as 4 characters
- Emoji: `"🚀"` should count as 1 character  
- Invalid UTF-8: Should return 0 characters (matching wc behavior)
- Mixed ASCII and multibyte content

### References
- [GNU wc source](https://github.com/coreutils/coreutils/blob/master/src/wc.c)
- [encoding_rs crate](https://crates.io/crates/encoding_rs)
- [POSIX locale specification](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/locale.h.html)