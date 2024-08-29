use std::fmt;

use chumsky::span::SimpleSpan;

pub mod headers;
mod lexer;

pub use lexer::lexer;
pub use lexer::Token;

type Span = SimpleSpan<usize>;

type Spanned<T> = (T, Span);
type ParserInput<'tok, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tok [(Token<'src>, Span)]>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(ident: String) -> Self {
        Ident(ident)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
