// use std::collections::HashMap;
use std::iter::Peekable;

use crate::lex;
use crate::parse::ParseError;

mod costumes;

// #[derive(Debug)]
// struct HeaderRegistry {
//     costumes: HashMap<String, costumes::Costume>,
// }

pub fn parse_header<T>(tokens: &mut Peekable<T>) -> Result<(), ParseError>
where
    T: Iterator<Item = lex::SpannedToken>,
{
    match tokens.peek().map(|t| &t.inner) {
        Some(&lex::Token::Keyword(lex::Keyword::Costumes)) => {
            costumes::parse_costumes_header(tokens)?;
        }
        _ => (),
    }

    Ok(())
}
