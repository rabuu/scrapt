use std::iter::Peekable;

use crate::lex;

pub use error::ParseError;

mod error;
mod header;

pub fn parse_tokens(tokens: impl IntoIterator<Item = lex::SpannedToken>) -> Result<(), ParseError> {
    let mut tokens: Peekable<_> = tokens.into_iter().peekable();

    while let Some(lex::Token::Keyword(kw)) = tokens.peek().map(|tok| &tok.inner) {
        if !kw.is_header() {
            return Ok(());
        }

        header::parse_header(&mut tokens)?;
    }

    Ok(())
}
