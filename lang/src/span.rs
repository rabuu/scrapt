/// Position in source file
///
/// interp. (line_number, column_number)
pub type SourcePosition = (usize, usize);

pub struct Span {
    pub begin: SourcePosition,
    pub end: Option<SourcePosition>,
}
