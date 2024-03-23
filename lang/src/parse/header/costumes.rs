use std::iter::Peekable;

use crate::lex::{Keyword, SpannedToken, Token};
use crate::media_type::ImgType;
use crate::parse::ParseError;

#[derive(Debug)]
pub struct Costume {
    current: bool,
    img_type: ImgType,
    path: String,
}

pub fn parse_costumes_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<(), ParseError> {
    let Some(header_type) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Keyword(Keyword::Costumes),
        });
    };

    if header_type.inner != Token::Keyword(Keyword::Costumes) {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Keyword(Keyword::Costumes),
            got: header_type.inner,
            span: header_type.span,
        });
    }

    let Some(open_curly) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::CurlyL,
        });
    };

    if open_curly.inner != Token::CurlyL {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::CurlyL,
            got: open_curly.inner,
            span: open_curly.span,
        });
    }

    loop {
        if tokens.peek().map(|t| &t.inner) == Some(&Token::CurlyR) {
            break;
        }

        let costume = parse_costume(tokens)?;
        eprintln!("{costume:?}");
    }

    let Some(closing_curly) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::CurlyR,
        });
    };

    if closing_curly.inner != Token::CurlyR {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::CurlyR,
            got: closing_curly.inner,
            span: closing_curly.span,
        });
    }

    Ok(())
}

pub fn parse_costume(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<Costume, ParseError> {
    // check for *
    let current = if tokens.peek().map(|t| &t.inner) == Some(&Token::Asterisk) {
        let _ = tokens.next();
        true
    } else {
        false
    };

    // expect name
    let Some(costume_name) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Ident(String::new()),
        });
    };

    let Token::Ident(costume_name) = costume_name.inner else {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Ident(String::new()),
            got: costume_name.inner,
            span: costume_name.span,
        });
    };

    // expect colon
    let Some(colon) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Colon,
        });
    };

    if colon.inner != Token::Colon {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Colon,
            got: colon.inner,
            span: colon.span,
        });
    }

    // expect image type
    let Some(img_type) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Keyword(Keyword::Img(ImgType::Svg)),
        });
    };

    let Token::Keyword(Keyword::Img(img_type)) = img_type.inner else {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Keyword(Keyword::Img(ImgType::Svg)),
            got: img_type.inner,
            span: img_type.span,
        });
    };

    // return if terminated
    if tokens.peek().map(|t| &t.inner) == Some(&Token::Semicolon) {
        let _ = tokens.next();
        return Ok(Costume {
            current,
            img_type,
            path: format!("{costume_name}.{}", img_type.file_extension()),
        });
    }

    // expect =
    let Some(equals) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Equal,
        });
    };

    if equals.inner != Token::Equal {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Equal,
            got: equals.inner,
            span: equals.span,
        });
    }

    // expect path
    let Some(path) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Str(String::new()),
        });
    };

    let Token::Str(path) = path.inner else {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Str(String::new()),
            got: path.inner,
            span: path.span,
        });
    };

    // expect semicolon
    let Some(semicolon) = tokens.next() else {
        return Err(ParseError::ExpectedTokenButEnd {
            expected: Token::Semicolon,
        });
    };

    if semicolon.inner != Token::Semicolon {
        return Err(ParseError::ExpectedAnotherToken {
            expected: Token::Semicolon,
            got: semicolon.inner,
            span: semicolon.span,
        });
    }

    Ok(Costume {
        current,
        img_type,
        path,
    })
}
