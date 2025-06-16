# TODO: Future Improvements

## Locale Handling for Word Counting

**Current State**: Using `is_ascii_whitespace()` for word boundary detection in `mwc`.

**Issue**: The original `wc` uses C's `isspace()` function which is locale-dependent. This means:
- In UTF-8 locales: Unicode whitespace characters (like non-breaking space U+00A0) are recognized
- In other locales: Different character sets define different whitespace characters
- Our current implementation only recognizes ASCII whitespace (space, tab, newline, CR, VT, FF)

**Examples of differences**:
- `wc` in UTF-8 locale would count Unicode spaces as word separators
- `wc` in Latin-1 locale would handle Latin-1 whitespace differently
- Our `mwc` only handles ASCII whitespace regardless of locale

**Investigation needed**:
1. Test `wc` behavior with various `LC_CTYPE` settings
2. Test with files containing Unicode whitespace characters
3. Research how to access locale information in Rust
4. Consider using `unicode-segmentation` crate for proper Unicode word boundaries

**Potential solutions**:
- Use `char::is_whitespace()` after UTF-8 conversion (assumes UTF-8 locale)
- Use libc bindings to call C's `isspace()` directly
- Implement locale-aware whitespace detection
- Document the limitation and provide a flag for strict ASCII mode

**Priority**: Low - most modern systems use UTF-8 locales, and ASCII whitespace covers 99% of use cases.