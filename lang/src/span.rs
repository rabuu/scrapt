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

/// Variable-length position in source file
///
/// Can be a `Single` position or a `Range`.
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
