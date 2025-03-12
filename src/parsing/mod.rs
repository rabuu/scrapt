use std::fmt;

use chumsky::Parser;
use chumsky::error::Rich;
use chumsky::input::Input;
use chumsky::span::SimpleSpan;

mod headers;
mod lexer;

pub use headers::Headers;

type Span = SimpleSpan;
type Spanned<T> = (T, Span);

type ParseErr<'src> = chumsky::extra::Err<Rich<'src, lexer::Token<'src>, Span>>;

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

pub fn parse(source: &str) -> Result<Headers, Vec<()>> {
    let (tokens, lex_errs) = lexer::lexer().parse(source).into_output_errors();

    let char_count = source.chars().count();
    let end_of_input: Span = (char_count..char_count).into();

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

// TODO
fn build_error(_err: Rich<String, Span>) -> () {
    todo!()
}
