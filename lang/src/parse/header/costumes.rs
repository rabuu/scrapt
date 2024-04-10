use std::collections::HashMap;
use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::media_type::ImgType;
use crate::parse::util::{expect_token, is_next_token};
use crate::parse::ParseError;
use crate::span::Span;

#[derive(Debug)]
pub struct Costume {
    pub img_type: ImgType,
    pub path: String,
}

pub fn parse_costumes_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
    costumes_db: &mut HashMap<String, Costume>,
    costumes_list: &mut Vec<String>,
    current_costume: &mut Option<u32>,
) -> Result<(), ParseError> {
    expect_token(tokens, TokenKind::CurlyL)?;

    loop {
        if is_next_token(tokens, TokenKind::CurlyR) {
            break;
        }

        let (name, name_span, costume, current) = parse_costume(tokens)?;

        if costumes_db.insert(name.clone(), costume).is_some() {
            return Err(ParseError::DuplicateHeaderValue {
                value: name,
                span: name_span,
            });
        }
        costumes_list.push(name);
        if let Some(span) = current {
            if current_costume.is_some() {
                return Err(ParseError::DuplicateSelection { span });
            }
            *current_costume = Some(costumes_list.len() as u32 - 1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex;

    #[test]
    fn valid_costumes_header() {
        let toks = lex::tokenize(r#"{ backdrop1: SVG; *bd2: PNG = "foo.PNG"; }"#).unwrap();
        let mut costumes_db = HashMap::new();
        let mut costumes_list = Vec::new();
        let mut current_costume = None;

        parse_costumes_header(
            &mut toks.into_iter().peekable(),
            &mut costumes_db,
            &mut costumes_list,
            &mut current_costume,
        )
        .unwrap();

        // TODO: verify results
    }
}
