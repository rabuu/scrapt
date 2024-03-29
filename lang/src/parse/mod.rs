use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
pub use error::ParseError;
pub use header::HeaderRegistry;

mod error;
mod header;
mod util;

pub fn parse_target(
    tokens: impl IntoIterator<Item = SpannedToken>,
) -> Result<HeaderRegistry, ParseError> {
    tracing::debug!("Start parsing...");

    let mut tokens: Peekable<_> = tokens.into_iter().peekable();
    let mut header_registry = HeaderRegistry::default();

    loop {
        if util::is_next_token(&mut tokens, TokenKind::Eof) {
            break;
        }

        header::parse_header(&mut tokens, &mut header_registry)?;
    }

    tracing::debug!("Successfully parsed target.");
    Ok(header_registry)
}
