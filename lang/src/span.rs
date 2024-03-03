/// Position in source file
///
/// interp. (line_number, column_number)
pub type SourcePosition = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    pub begin: SourcePosition,
    pub end: Option<SourcePosition>,
}

impl Span {
    pub fn single(begin: SourcePosition) -> Self {
        Span { begin, end: None }
    }

    pub fn range(begin: SourcePosition, end: SourcePosition) -> Self {
        Span {
            begin,
            end: Some(end),
        }
    }
}
