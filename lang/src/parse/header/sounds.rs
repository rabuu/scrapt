use std::collections::HashMap;
use std::iter::Peekable;

use crate::lex::{SpannedToken, TokenKind};
use crate::media_type::AudioType;
use crate::parse::util::{expect_token, is_next_token};
use crate::parse::ParseError;
use crate::span::Span;

#[derive(Debug)]
pub struct Sound {
    pub audio_type: AudioType,
    pub path: String,
}

pub fn parse_sounds_header(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
    sounds_db: &mut HashMap<String, Sound>,
    sounds_list: &mut Vec<String>,
) -> Result<(), ParseError> {
    expect_token(tokens, TokenKind::CurlyL)?;

    loop {
        if is_next_token(tokens, TokenKind::CurlyR) {
            break;
        }

        let (name, name_span, sound) = parse_sound(tokens)?;

        if sounds_db.insert(name.clone(), sound).is_some() {
            return Err(ParseError::DuplicateHeaderValue {
                value: name,
                span: name_span,
            });
        }
        sounds_list.push(name);
    }

    expect_token(tokens, TokenKind::CurlyR)?;

    Ok(())
}

fn parse_sound(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken>>,
) -> Result<(String, Span, Sound), ParseError> {
    let sound_name_token = expect_token(tokens, TokenKind::Ident)?;
    let name_span = sound_name_token.span;
    let sound_name = sound_name_token.inner.try_to_inner_string().unwrap();

    expect_token(tokens, TokenKind::Colon)?;

    let audio_type = expect_token(tokens, TokenKind::AudioType)?
        .inner
        .try_to_inner_audio_type()
        .unwrap();

    // return if terminated
    if is_next_token(tokens, TokenKind::Semicolon) {
        let _ = tokens.next();
        let path = format!("{sound_name}.{}", audio_type.file_extension());
        return Ok((sound_name, name_span, Sound { audio_type, path }));
    }

    expect_token(tokens, TokenKind::Equal)?;

    let path = expect_token(tokens, TokenKind::Str)?
        .inner
        .try_to_inner_string()
        .unwrap();

    expect_token(tokens, TokenKind::Semicolon)?;

    Ok((sound_name, name_span, Sound { audio_type, path }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex;

    #[test]
    fn valid_sounds_header() {
        let toks = lex::tokenize(r#"{ sound1: WAV; sound2: MP4 = "foo.MP4"; }"#).unwrap();
        let mut sounds_db = HashMap::new();
        let mut sounds_list = Vec::new();

        parse_sounds_header(
            &mut toks.into_iter().peekable(),
            &mut sounds_db,
            &mut sounds_list,
        )
        .unwrap();

        // TODO: verify results
    }
}
