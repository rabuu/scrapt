use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexerError {
    #[error("unterminated string literal at {span}")]
    UnterminatedStringLiteral { span: Span },

    #[error("illegal identifier `{ident}` at {span}")]
    IllegalIdent { ident: String, span: Span },

    #[error("non-number starting with numerical character `{word}` at {span}")]
    BeginsWithNumber { word: String, span: Span },

    #[error("illegal character `{c}` at {span}")]
    IllegalChar { c: char, span: Span },
}
