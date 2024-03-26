use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::media_type::ImgType;
use crate::parse::{expect_token, is_next_token, ParseError};
use crate::span::Span;

use super::registry::HeaderRegistry;

#[derive(Debug)]
pub struct Costume {
    img_type: ImgType,
    path: String,
}

pub fn parse_costumes_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
    registry: &mut HeaderRegistry,
) -> Result<(), ParseError> {
    expect_token(tokens, TokenKind::Costumes)?;
    expect_token(tokens, TokenKind::CurlyL)?;

    loop {
        if is_next_token(tokens, TokenKind::CurlyR) {
            break;
        }

        let (name, name_span, costume, current) = parse_costume(tokens)?;

        if registry.costumes.insert(name.clone(), costume).is_some() {
            return Err(ParseError::DuplicateHeaderValue {
                value: name,
                span: name_span,
            });
        }
        registry.costumes_list.push(name);
        if let Some(span) = current {
            if registry.current_costume.is_some() {
                return Err(ParseError::DuplicateSelection { span });
            }
            registry.current_costume = Some(registry.costumes_list.len() - 1);
        }
    }

    expect_token(tokens, TokenKind::CurlyR)?;

    Ok(())
}

fn parse_costume(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<(String, Span, Costume, Option<Span>), ParseError> {
    // check for *
    let current = if is_next_token(tokens, TokenKind::Asterisk) {
        let tok = expect_token(tokens, TokenKind::Asterisk)?;
        Some(tok.span)
    } else {
        None
    };

    let costume_name_token = expect_token(tokens, TokenKind::Ident)?;
    let name_span = costume_name_token.span;
    let costume_name = costume_name_token.inner.try_to_inner_string().unwrap();

    expect_token(tokens, TokenKind::Colon)?;

    let img_type = expect_token(tokens, TokenKind::ImgType)?
        .inner
        .try_to_inner_img_type()
        .unwrap();

    // return if terminated
    if is_next_token(tokens, TokenKind::Semicolon) {
        let _ = tokens.next();
        let path = format!("{costume_name}.{}", img_type.file_extension());
        return Ok((costume_name, name_span, Costume { img_type, path }, current));
    }

    expect_token(tokens, TokenKind::Equal)?;

    let path = expect_token(tokens, TokenKind::Str)?
        .inner
        .try_to_inner_string()
        .unwrap();

    expect_token(tokens, TokenKind::Semicolon)?;

    Ok((costume_name, name_span, Costume { img_type, path }, current))
}
