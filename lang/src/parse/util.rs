use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::parse::ParseError;

pub fn expect_token(
    tokens: &mut impl Iterator<Item = SpannedToken>,
    expected: TokenKind,
) -> Result<SpannedToken, ParseError> {
    let Some(tok) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd { expected });
    };

    if tok.inner.kind() != expected {
        return Err(ParseError::ExpectedAnotherToken {
            expected,
            got: tok.inner,
            span: tok.span,
        });
    }

    Ok(tok)
}

pub fn is_next_token(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
    token_kind: TokenKind,
) -> bool {
    tokens.peek().map(|t| t.inner.kind()) == Some(token_kind)
}
