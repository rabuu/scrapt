use thiserror::Error;

use crate::{lex, span};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected token `{expected}` but parsing ended")]
    ExpectedTokenButEnd { expected: lex::TokenKind },

    #[error("expected token `{expected}` but got `{}` at {span}", got.kind())]
    ExpectedAnotherToken {
        expected: lex::TokenKind,
        got: lex::Token,
        span: span::Span,
    },

    #[error("duplicate definition of header value `{value}` at {span}")]
    DuplicateHeaderValue { value: String, span: span::Span },

    #[error("duplicate selection using * at {span}")]
    DuplicateSelection { span: span::Span },
}
