use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::media_type::ImgType;
use crate::parse::{expect_token, is_next_token, ParseError};

#[derive(Debug)]
pub struct Costume {
    current: bool,
    img_type: ImgType,
    path: String,
}

pub fn parse_costumes_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<(), ParseError> {
    expect_token(tokens, TokenKind::Costumes)?;
    expect_token(tokens, TokenKind::CurlyL)?;

    loop {
        if is_next_token(tokens, TokenKind::CurlyR) {
            break;
        }

        let costume = parse_costume(tokens)?;
        eprintln!("{costume:?}");
    }

    expect_token(tokens, TokenKind::CurlyR)?;

    Ok(())
}

pub fn parse_costume(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<Costume, ParseError> {
    // check for *
    let current = is_next_token(tokens, TokenKind::Asterisk);
    if current {
        let _ = tokens.next();
    }

    let costume_name = expect_token(tokens, TokenKind::Ident)?
        .inner
        .try_to_inner_string()
        .unwrap();

    expect_token(tokens, TokenKind::Colon)?;

    let img_type = expect_token(tokens, TokenKind::ImgType)?
        .inner
        .try_to_inner_img_type()
        .unwrap();

    // return if terminated
    if is_next_token(tokens, TokenKind::Semicolon) {
        let _ = tokens.next();
        return Ok(Costume {
            current,
            img_type,
            path: format!("{costume_name}.{}", img_type.file_extension()),
        });
    }

    expect_token(tokens, TokenKind::Equal)?;

    let path = expect_token(tokens, TokenKind::Str)?
        .inner
        .try_to_inner_string()
        .unwrap();

    expect_token(tokens, TokenKind::Semicolon)?;

    Ok(Costume {
        current,
        img_type,
        path,
    })
}
