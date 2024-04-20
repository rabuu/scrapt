use std::collections::HashMap;
use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::parse::util::{expect_token, is_next_token};
use crate::parse::ParseError;
use crate::span::Span;

#[derive(Debug)]
pub enum SetValue {
    Int(u32),
    Float(f32),
    Str(String),
}

pub fn parse_set_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
    set_db: &mut HashMap<String, SetValue>,
) -> Result<(), ParseError> {
    expect_token(tokens, TokenKind::CurlyL)?;

    loop {
        if is_next_token(tokens, TokenKind::CurlyR) {
            break;
        }

        let (key, key_span, value) = parse_key(tokens)?;

        if set_db.insert(key.clone(), value).is_some() {
            return Err(ParseError::DuplicateHeaderValue {
                value: key,
                span: key_span,
            });
        }
    }

    expect_token(tokens, TokenKind::CurlyR)?;

    Ok(())
}

fn parse_key(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<(String, Span, SetValue), ParseError> {
    let key_token = expect_token(tokens, TokenKind::Ident)?;
    let key_span = key_token.span;
    let key = key_token.inner.try_to_inner_string().unwrap();

    expect_token(tokens, TokenKind::Equal)?;

    let value = expect_token(tokens, TokenKind::Value)?;
    todo!();

    expect_token(tokens, TokenKind::Semicolon)?;

    Ok((key, key_span, value))
}

// TODO: tests
