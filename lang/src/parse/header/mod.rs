use std::iter::Peekable;

use crate::lex::{SpannedToken, Token, TokenKind};
use crate::parse::util::expect_token;
use crate::parse::ParseError;

pub use registry::HeaderRegistry;

mod costumes;
mod registry;

pub fn parse_header<T>(
    tokens: &mut Peekable<T>,
    registry: &mut HeaderRegistry,
) -> Result<(), ParseError>
where
    T: Iterator<Item = SpannedToken>,
{
    let header = expect_token(tokens, TokenKind::Header)?;

    #[allow(clippy::single_match)]
    match header.inner {
        Token::Costumes => costumes::parse_costumes_header(
            tokens,
            &mut registry.costumes,
            &mut registry.costumes_list,
            &mut registry.current_costume,
        )?,
        _ => (),
    }

    Ok(())
}
