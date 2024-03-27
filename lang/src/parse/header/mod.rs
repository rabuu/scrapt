use std::iter::Peekable;

use crate::lex::{SpannedToken, Token};
use crate::parse::ParseError;

mod costumes;
mod registry;

pub fn parse_header<T>(tokens: &mut Peekable<T>) -> Result<(), ParseError>
where
    T: Iterator<Item = SpannedToken>,
{
    let mut registry = registry::HeaderRegistry::default();

    #[allow(clippy::single_match)]
    match tokens.peek().map(|t| &t.inner) {
        Some(&Token::Costumes) => {
            costumes::parse_costumes_header(tokens, &mut registry)?;
        }
        _ => (),
    }

    dbg!(registry);

    Ok(())
}
