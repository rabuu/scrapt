use std::iter::Peekable;

use tracing::trace;

use crate::lex::SpannedToken;
pub use error::ParseError;
pub use header::HeaderRegistry;

mod error;
mod header;
mod util;

pub fn parse_target(
    tokens: impl IntoIterator<Item = SpannedToken>,
) -> Result<HeaderRegistry, ParseError> {
    let mut tokens: Peekable<_> = tokens.into_iter().peekable();

    let mut header_registry = HeaderRegistry::default();

    while let Some(tok) = tokens.peek().map(|tok| &tok.inner) {
        if !tok.is_header() {
            break;
        }

        header::parse_header(&mut tokens, &mut header_registry)?;
    }

    trace!("Successfully parsed target.");
    Ok(header_registry)
}
