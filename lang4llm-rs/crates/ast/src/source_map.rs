//! SourceMap: byte offset → line:col conversion
//!
//! Precomputes a table of line-start byte offsets during construction,
//! then uses binary search for O(log L) lookup.

use crate::Span;

/// Precomputed byte-offset-to-line:col converter.
///
/// Build once per source string, then call [`SourceMap::span`] from grammar
/// actions using LALRPOP `@L`/`@R` byte offsets.
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// Byte offset of the start of each line (0-indexed line → byte offset).
    /// `line_starts[0]` is always 0 (start of line 1).
    line_starts: Vec<usize>,
    /// Total length of the source in bytes (for bounds checking).
    source_len: usize,
}

impl SourceMap {
    /// Build a SourceMap from a source string.
    ///
    /// Runs in O(n) where n = `source.len()`.
    pub fn from_source(source: &str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + ch.len_utf8());
            }
        }
        Self {
            line_starts,
            source_len: source.len(),
        }
    }

    /// Convert a byte offset to `(line, col)` — both 1-indexed.
    ///
    /// - `offset` must be in `0..=source_len` (inclusive for EOF).
    /// - If `offset` is out of bounds it is clamped to `source_len`.
    /// - Column is a byte offset within the line, not a Unicode character
    ///   count (consistent with LALRPOP which works in bytes).
    pub fn resolve(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.source_len);
        // Binary search for the last line_start <= offset
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(exact) => exact,          // offset is the first byte of a line
            Err(insertion) => insertion - 1, // offset is inside a line
        };
        let line = line_idx + 1; // 1-indexed
        let col = offset - self.line_starts[line_idx] + 1; // 1-indexed
        (line, col)
    }

    /// Build a [`Span`] from two LALRPOP byte offsets (`@L`, `@R`).
    pub fn span(&self, start: usize, end: usize) -> Span {
        let (sl, sc) = self.resolve(start);
        let (el, ec) = self.resolve(end);
        Span::new(sl, sc, el, ec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let sm = SourceMap::from_source("");
        assert_eq!(sm.resolve(0), (1, 1));
    }

    #[test]
    fn single_line_no_newline() {
        let sm = SourceMap::from_source("hello");
        assert_eq!(sm.resolve(0), (1, 1)); // 'h'
        assert_eq!(sm.resolve(4), (1, 5)); // 'o'
        assert_eq!(sm.resolve(5), (1, 6)); // EOF
    }

    #[test]
    fn multi_line() {
        // "ab\ncd\nef"
        // line 1: bytes 0-2 (ab\n)
        // line 2: bytes 3-5 (cd\n)
        // line 3: bytes 6-7 (ef)
        let sm = SourceMap::from_source("ab\ncd\nef");
        assert_eq!(sm.resolve(0), (1, 1)); // 'a'
        assert_eq!(sm.resolve(1), (1, 2)); // 'b'
        assert_eq!(sm.resolve(2), (1, 3)); // '\n'
        assert_eq!(sm.resolve(3), (2, 1)); // 'c'
        assert_eq!(sm.resolve(4), (2, 2)); // 'd'
        assert_eq!(sm.resolve(5), (2, 3)); // '\n'
        assert_eq!(sm.resolve(6), (3, 1)); // 'e'
        assert_eq!(sm.resolve(7), (3, 2)); // 'f'
        assert_eq!(sm.resolve(8), (3, 3)); // EOF
    }

    #[test]
    fn utf8_multibyte() {
        // "α\nβ" — α is 2 bytes (CE B1), β is 2 bytes (CE B2)
        // byte 0-1: 'α', byte 2: '\n', byte 3-4: 'β'
        let s = "α\nβ";
        let sm = SourceMap::from_source(s);
        assert_eq!(s.len(), 5);
        assert_eq!(sm.resolve(0), (1, 1)); // start of α
        assert_eq!(sm.resolve(2), (1, 3)); // '\n'
        assert_eq!(sm.resolve(3), (2, 1)); // start of β
        assert_eq!(sm.resolve(5), (2, 3)); // EOF
    }

    #[test]
    fn boundary_first_byte_of_line() {
        let sm = SourceMap::from_source("a\nb\nc");
        // 'a'=0, '\n'=1, 'b'=2, '\n'=3, 'c'=4
        assert_eq!(sm.resolve(2), (2, 1));
        assert_eq!(sm.resolve(4), (3, 1));
    }

    #[test]
    fn out_of_bounds_clamped() {
        let sm = SourceMap::from_source("hi");
        // offset 999 should clamp to source_len=2
        assert_eq!(sm.resolve(999), sm.resolve(2));
    }

    #[test]
    fn span_construction() {
        let sm = SourceMap::from_source("ab\ncd");
        let s = sm.span(0, 4);
        assert_eq!(s.start_line, 1);
        assert_eq!(s.start_col, 1);
        assert_eq!(s.end_line, 2);
        assert_eq!(s.end_col, 2);
    }
}
