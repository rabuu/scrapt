use std::iter::Peekable;

use crate::lex;

pub use error::ParseError;

mod error;
mod header;

pub fn parse_tokens(tokens: impl IntoIterator<Item = lex::SpannedToken>) -> Result<(), ParseError> {
    let mut tokens: Peekable<_> = tokens.into_iter().peekable();

    while let Some(tok) = tokens.peek().map(|tok| &tok.inner) {
        if !tok.is_header() {
            return Ok(());
        }

        header::parse_header(&mut tokens)?;
    }

    Ok(())
}

fn expect_token(
    tokens: &mut impl Iterator<Item = lex::SpannedToken>,
    expected: lex::TokenKind,
) -> Result<lex::SpannedToken, ParseError> {
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

fn is_next_token(
    tokens: &mut Peekable<impl Iterator<Item = lex::SpannedToken>>,
    token_kind: lex::TokenKind,
) -> bool {
    tokens.peek().map(|t| t.inner.kind()) == Some(token_kind)
}
