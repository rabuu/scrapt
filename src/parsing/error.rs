use chumsky::error::{Rich, RichReason};
use miette::Diagnostic;
use thiserror::Error;

use super::Span;

#[derive(Debug, Error, Diagnostic)]
pub enum ParsingError {
    #[error("Expected one of {expected} but got {found}")]
    Unexpected {
        expected: String,
        found: String,

        #[label("unexpected token")]
        span: Span,
    },

    #[error("{msg}")]
    Custom {
        msg: String,

        #[label("here")]
        span: Span,
    },
}

pub(super) fn build_error(err: Rich<String, Span>) -> ParsingError {
    match err.reason() {
        RichReason::ExpectedFound { expected, found } => {
            let expected: Vec<String> = expected.iter().map(|x| x.to_string()).collect();
            let found: String = match found {
                None => "EOF".to_string(),
                Some(found) => found.to_string(),
            };

            ParsingError::Unexpected {
                expected: string_list(&expected),
                found,
                span: *err.span(),
            }
        }
        RichReason::Custom(msg) => ParsingError::Custom {
            msg: msg.to_string(),
            span: *err.span(),
        },
    }
}

fn string_list(strings: &[String]) -> String {
    let len = strings.len();
    let mut builder = String::new();
    for (i, s) in strings.iter().enumerate() {
        builder.push_str(s);
        if i < len - 1 {
            builder.push_str(", ");
        }
    }
    builder
}
