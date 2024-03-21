//! Span module
//!
//! This module exposes the [SourcePosition] and [Span] types for working with positions in the
//! source file.

use std::fmt::{self, Display};

/// Atomic position in source file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl SourcePosition {
    pub fn new(row: usize, col: usize) -> Self {
        SourcePosition { row, col }
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

/// Variable-length position in source file
///
/// Can be a [single](Span::Single) position or a [range](Span::Range).
#[derive(Debug, PartialEq, Eq)]
pub enum Span {
    Single(SourcePosition),
    Range {
        begin: SourcePosition,
        end: SourcePosition,
    },
}

impl Span {
    pub fn single(pos: SourcePosition) -> Self {
        Span::Single(pos)
    }

    pub fn range(begin: SourcePosition, end: SourcePosition) -> Self {
        Span::Range { begin, end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Span::Single(pos) => write!(f, "{pos}"),
            Span::Range { begin, end } => write!(f, "{begin}-{end}"),
        }
    }
}
