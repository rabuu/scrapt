//! Lexing module
//!
//! This module contains the lexing logic of the language.
//! The main way of interacting with the lexer is using the [tokenize] function.

use crate::{
    media_type::{AudioType, ImgType},
    span::Span,
};

use cursor::Cursor;
pub use error::LexError;
pub use token::{SpannedToken, Token, TokenKind};

mod cursor;
mod error;
mod token;

/// Tokenize a source string
///
/// This function is the interface of the lexer.
/// It takes a source string and returns the found [Token]s of the valid source file.
///
/// # Errors
/// The function returns only the *first* [LexError].
pub fn tokenize(source: impl AsRef<str>) -> Result<Vec<SpannedToken>, LexError> {
    tracing::debug!("Start tokenizing...");

    let mut cursor = Cursor::new(source.as_ref());
    let mut tokens = Vec::new();

    loop {
        let token = cursor.advance_token()?;
        let is_eof = token == Token::Eof;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    tracing::debug!("Successfully tokenized file.");
    Ok(tokens)
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Result<SpannedToken, LexError> {
        use Token::*;

        self.eat(|c| c.is_whitespace());

        let begin = self.position();
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Ok(Eof.position(begin)),
        };

        match first_char {
            // slash or comment
            '/' => match self.peek_this() {
                '/' => match self.peek_next() {
                    '/' => {
                        self.bump();
                        self.bump();
                        Ok(MetaComment(self.eat(|c| c != '\n').trim().to_string())
                            .span(begin, self.prev_position()))
                    }
                    _ => {
                        self.bump();
                        Ok(Comment(self.eat(|c| c != '\n').trim().to_string())
                            .span(begin, self.prev_position()))
                    }
                },
                _ => Ok(Slash.position(begin)),
            },

            // one symbol tokens
            '+' => Ok(Plus.position(begin)),
            '=' => Ok(Equal.position(begin)),
            ',' => Ok(Comma.position(begin)),
            '*' => Ok(Asterisk.position(begin)),
            ';' => Ok(Semicolon.position(begin)),
            '(' => Ok(ParenL.position(begin)),
            ')' => Ok(ParenR.position(begin)),
            '{' => Ok(CurlyL.position(begin)),
            '}' => Ok(CurlyR.position(begin)),
            '[' => Ok(BracketL.position(begin)),
            ']' => Ok(BracketR.position(begin)),
            '<' => Ok(ChevronL.position(begin)),
            '>' => Ok(ChevronR.position(begin)),

            // colons
            ':' => match self.peek_this() {
                ':' => {
                    self.bump();
                    Ok(DoubleColon.span(begin, self.prev_position()))
                }
                _ => Ok(Colon.position(begin)),
            },

            // minus or arrow
            '-' => match self.peek_this() {
                '>' => {
                    self.bump();
                    Ok(Arrow.span(begin, self.prev_position()))
                }
                _ => Ok(Minus.position(begin)),
            },

            // strings
            '"' => {
                let string = self.eat(|c| c != '"');
                let closing_del = self.bump();

                if closing_del != Some('"') {
                    return Err(LexError::UnterminatedStringLiteral {
                        span: Span::range(begin, self.prev_position()),
                    });
                }

                Ok(Str(string).span(begin, self.prev_position()))
            }

            // raw idents
            'r' if self.peek_this() == '#' && self.peek_next().is_ascii_alphabetic() => {
                self.bump();
                let ident = self.eat(|c| c.is_ascii_alphanumeric());

                if ident.is_empty() {
                    return Err(LexError::IllegalIdent {
                        ident,
                        span: Span::range(begin, self.prev_position()),
                    });
                }

                Ok(Ident(ident).span(begin, self.prev_position()))
            }

            // numerics
            c if c.is_ascii_digit() => {
                let inp = self.eat_with_prev(|c| c.is_ascii_digit() || c == '.');

                if let Ok(int) = inp.parse::<usize>() {
                    Ok(Int(int).span(begin, self.prev_position()))
                } else if let Ok(float) = inp.parse::<f64>() {
                    Ok(Float(float).span(begin, self.prev_position()))
                } else {
                    Err(LexError::BeginsWithNumber {
                        word: inp,
                        span: Span::range(begin, self.prev_position()),
                    })
                }
            }

            // keywords and idents
            c if c.is_ascii_alphabetic() => {
                let inp = self.eat_with_prev(|c| c.is_ascii_alphanumeric());
                let kw = match inp.as_str() {
                    "global" => Global,
                    "vars" => Vars,
                    "lists" => Lists,
                    "broadcasts" => Broadcasts,
                    "costumes" => Costumes,
                    "sounds" => Sounds,
                    "SVG" => Img(ImgType::Svg),
                    "PNG" => Img(ImgType::Png),
                    "WAV" => Audio(AudioType::Wav),
                    "MP4" => Audio(AudioType::Mp4),
                    "def" => Def,
                    "repeat" => Repeat,
                    "if" => If,
                    "else" => Else,
                    _ => Ident(inp),
                };

                Ok(kw.span(begin, self.prev_position()))
            }

            c => Err(LexError::IllegalChar {
                c,
                span: Span::single(begin),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::SourcePosition;

    #[test]
    fn simple_tokenization() {
        use Token::*;

        let input = "+-=,/*:;::->(){}[]<>";
        assert_eq!(
            tokenize(input).unwrap(),
            vec![
                Plus,
                Minus,
                Equal,
                Comma,
                Slash,
                Asterisk,
                Colon,
                Semicolon,
                DoubleColon,
                Arrow,
                ParenL,
                ParenR,
                CurlyL,
                CurlyR,
                BracketL,
                BracketR,
                ChevronL,
                ChevronR,
                Eof
            ]
        );
    }

    #[test]
    fn keyword_tokenization() {
        let input = "global vars lists broadcasts foo costumes if def";
        assert_eq!(
            tokenize(input).unwrap(),
            vec![
                Token::Global,
                Token::Vars,
                Token::Lists,
                Token::Broadcasts,
                Token::Ident(String::from("foo")),
                Token::Costumes,
                Token::If,
                Token::Def,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn unterminated_string() {
        let input = r#"string: "halllo"#;
        let err = tokenize(input).unwrap_err();
        assert_eq!(
            err,
            LexError::UnterminatedStringLiteral {
                span: Span::range(SourcePosition::new(1, 9), SourcePosition::new(1, 15))
            }
        )
    }

    #[test]
    fn illegal_character() {
        let input = "ü";
        let err = tokenize(input).unwrap_err();

        assert_eq!(
            err,
            LexError::IllegalChar {
                c: 'ü',
                span: Span::single(SourcePosition::new(1, 1))
            }
        );
    }
}
