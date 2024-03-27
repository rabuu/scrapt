use std::iter::Peekable;

use crate::lex::{SpannedToken, Token};
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
    #[allow(clippy::single_match)]
    match tokens.peek().map(|t| &t.inner) {
        Some(&Token::Costumes) => {
            costumes::parse_costumes_header(
                tokens,
                &mut registry.costumes,
                &mut registry.costumes_list,
                &mut registry.current_costume,
            )?;
        }
        _ => (),
    }

    Ok(())
}
