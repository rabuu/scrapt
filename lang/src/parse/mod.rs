use std::iter::Peekable;

use crate::lex::SpannedToken;

pub use error::ParseError;

mod error;
mod header;
mod util;

pub fn parse_tokens(tokens: impl IntoIterator<Item = SpannedToken>) -> Result<(), ParseError> {
    let mut tokens: Peekable<_> = tokens.into_iter().peekable();

    while let Some(tok) = tokens.peek().map(|tok| &tok.inner) {
        if !tok.is_header() {
            return Ok(());
        }

        header::parse_header(&mut tokens)?;
    }

    Ok(())
}
