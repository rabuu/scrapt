use std::iter::Peekable;

use crate::lex::{SpannedToken, Token, TokenKind};
use crate::parse::util::expect_token;
use crate::parse::ParseError;

pub use registry::HeaderRegistry;

mod costumes;
mod registry;
mod sounds;

pub fn parse_header<T>(
    tokens: &mut Peekable<T>,
    registry: &mut HeaderRegistry,
) -> Result<(), ParseError>
where
    T: Iterator<Item = SpannedToken>,
{
    let header = expect_token(tokens, TokenKind::Header)?;

    match header.inner {
        Token::Costumes => costumes::parse_costumes_header(
            tokens,
            &mut registry.costumes,
            &mut registry.costumes_list,
            &mut registry.current_costume,
        )?,
        Token::Sounds => {
            sounds::parse_sounds_header(tokens, &mut registry.sounds, &mut registry.sounds_list)?
        }
        _ => (),
    }

    Ok(())
}
