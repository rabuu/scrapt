mod error;
mod headers;
mod lexer;
mod span;

use std::fmt;

use chumsky::input::ValueInput;
use chumsky::prelude::*;

pub use error::ParsingError;
use error::build_error;
pub use headers::Headers;
use lexer::Token;
pub use span::Span;

type ParseErr<'src> = chumsky::extra::Err<Rich<'src, lexer::Token<'src>, Span>>;

pub trait ParseInput<'src>: ValueInput<'src, Token = Token<'src>, Span = Span> {}
impl<'src, T: ValueInput<'src, Token = Token<'src>, Span = Span>> ParseInput<'src> for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(ident: String) -> Self {
        Ident(ident)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse(source: &str) -> Result<Headers, Vec<ParsingError>> {
    let (tokens, lex_errs) = lexer::lexer()
        .parse(source.map_span(Into::into))
        .into_output_errors();

    let char_count = source.chars().count();
    let end_of_input: Span = Span::marker(char_count);

    let parse_errs = if let Some(tokens) = &tokens {
        let (headers, parse_errs) = Headers::parser()
            .parse(tokens.as_slice().map(end_of_input, |(t, s)| (t, s)))
            .into_output_errors();

        if let Some(headers) = headers.filter(|_| lex_errs.len() + parse_errs.len() == 0) {
            return Ok(headers);
        }

        parse_errs
    } else {
        Vec::new()
    };

    let all_errors = lex_errs
        .into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|tok| tok.to_string())),
        )
        .map(build_error)
        .collect();

    Err(all_errors)
}
