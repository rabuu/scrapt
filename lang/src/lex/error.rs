//! Lexing error module

use thiserror::Error;

use crate::span::Span;

/// Error type for the lexing step
///
/// Probably returned by [tokenize](crate::lex::tokenize).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexError {
    #[error("unterminated string literal at {span}")]
    UnterminatedStringLiteral { span: Span },

    #[error("illegal identifier `{ident}` at {span}")]
    IllegalIdent { ident: String, span: Span },

    #[error("non-number starting with numerical character `{word}` at {span}")]
    BeginsWithNumber { word: String, span: Span },

    #[error("illegal character `{c}` at {span}")]
    IllegalChar { c: char, span: Span },
}
