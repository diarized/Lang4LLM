//! Source location tracking

use serde::{Deserialize, Serialize};
use std::fmt;

/// A span represents a location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Start line (1-indexed)
    pub start_line: usize,
    /// Start column (1-indexed)
    pub start_col: usize,
    /// End line (1-indexed)
    pub end_line: usize,
    /// End column (1-indexed)
    pub end_col: usize,
}

impl Span {
    /// Create a new span
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    /// Create a dummy span (for testing)
    pub fn dummy() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Combine two spans into one covering both
    pub fn merge(self, other: Self) -> Self {
        Self {
            start_line: self.start_line.min(other.start_line),
            start_col: if self.start_line == other.start_line {
                self.start_col.min(other.start_col)
            } else if self.start_line < other.start_line {
                self.start_col
            } else {
                other.start_col
            },
            end_line: self.end_line.max(other.end_line),
            end_col: if self.end_line == other.end_line {
                self.end_col.max(other.end_col)
            } else if self.end_line > other.end_line {
                self.end_col
            } else {
                other.end_col
            },
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}
