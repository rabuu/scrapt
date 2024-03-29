use thiserror::Error;

use crate::lex::{Token, TokenKind};
use crate::span::Span;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected token `{expected}` but parsing ended")]
    ExpectedTokenButEnd { expected: TokenKind },

    #[error("expected token `{expected}` but got `{}` at {span}", got.kind())]
    ExpectedAnotherToken {
        expected: TokenKind,
        got: Token,
        span: Span,
    },

    #[error("duplicate definition of header value `{value}` at {span}")]
    DuplicateHeaderValue { value: String, span: Span },

    #[error("duplicate selection using * at {span}")]
    DuplicateSelection { span: Span },
}
