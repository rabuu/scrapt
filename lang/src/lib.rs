use std::fmt;

use chumsky::error::Rich;
use chumsky::span::SimpleSpan;

pub mod headers;
mod lexer;

pub use lexer::{lexer, Token};

type Span = SimpleSpan;
type Spanned<T> = (T, Span);

type ParserError<'src> = chumsky::extra::Err<Rich<'src, Token<'src>, Span>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(ident: String) -> Self {
        Ident(ident)
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
