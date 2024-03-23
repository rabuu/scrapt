use thiserror::Error;

use crate::{lex, span};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected token `{expected}` but parsing ended")]
    ExpectedTokenButEnd { expected: lex::Token },

    #[error("expected token `{expected}` but got `{got}` at {span}")]
    ExpectedAnotherToken {
        expected: lex::Token,
        got: lex::Token,
        span: span::Span,
    },
}
