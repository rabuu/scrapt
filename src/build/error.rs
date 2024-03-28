use thiserror::Error;

use lang::lex::LexError;
use lang::parse::ParseError;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("lexing unsuccessful")]
    LexError(#[from] LexError),

    #[error("parsing unsuccessful")]
    ParseError(#[from] ParseError),
}
