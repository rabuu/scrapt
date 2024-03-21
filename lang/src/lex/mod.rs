pub use cursor::Cursor;

use crate::span::Span;

pub use error::LexerError;
pub use token::{Keyword, Token};

mod cursor;
mod error;
mod token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = cursor.advance_token()?;
        let is_eof = token == Token::Eof;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    Ok(tokens)
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Result<Token, LexerError> {
        use Token::*;

        self.eat(|c| c.is_whitespace());

        let begin = self.position();
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Ok(Eof),
        };

        match first_char {
            // slash or comment
            '/' => match self.peek_this() {
                '/' => match self.peek_next() {
                    '/' => {
                        self.bump();
                        self.bump();
                        Ok(MetaComment(self.eat(|c| c != '\n').trim().to_string()))
                    }
                    _ => {
                        self.bump();
                        Ok(Comment(self.eat(|c| c != '\n').trim().to_string()))
                    }
                },
                _ => Ok(Slash),
            },

            // one symbol tokens
            '+' => Ok(Plus),
            '=' => Ok(Equal),
            ',' => Ok(Comma),
            '*' => Ok(Asterisk),
            ';' => Ok(Semicolon),
            '(' => Ok(ParenL),
            ')' => Ok(ParenR),
            '{' => Ok(CurlyL),
            '}' => Ok(CurlyR),
            '[' => Ok(BracketL),
            ']' => Ok(BracketR),
            '<' => Ok(ChevronL),
            '>' => Ok(ChevronR),

            // colons
            ':' => match self.peek_this() {
                ':' => {
                    self.bump();
                    Ok(DoubleColon)
                }
                _ => Ok(Colon),
            },

            // minus or arrow
            '-' => match self.peek_this() {
                '>' => {
                    self.bump();
                    Ok(Arrow)
                }
                _ => Ok(Minus),
            },

            // strings
            '"' => {
                let string = self.eat(|c| c != '"');
                let closing_del = self.bump();

                if closing_del != Some('"') {
                    return Err(LexerError::UnterminatedStringLiteral {
                        span: Span::range(begin, self.prev_position()),
                    });
                }

                Ok(Str(string))
            }

            // raw idents
            'r' if self.peek_this() == '#' && self.peek_next().is_ascii_alphabetic() => {
                self.bump();
                let ident = self.eat(|c| c.is_ascii_alphanumeric());

                if ident.is_empty() {
                    return Err(LexerError::IllegalIdent {
                        ident,
                        span: Span::range(begin, self.prev_position()),
                    });
                }

                Ok(Ident(ident))
            }

            // numerics
            c if c.is_ascii_digit() => {
                let inp = self.eat_with_prev(|c| c.is_ascii_digit() || c == '.');

                if let Ok(int) = inp.parse::<usize>() {
                    Ok(Int(int))
                } else if let Ok(float) = inp.parse::<f64>() {
                    Ok(Float(float))
                } else {
                    Err(LexerError::BeginsWithNumber {
                        word: inp,
                        span: Span::range(begin, self.prev_position()),
                    })
                }
            }

            // keywords and idents
            c if c.is_ascii_alphabetic() => {
                use self::token::Keyword::*;

                let inp = self.eat_with_prev(|c| c.is_ascii_alphanumeric());
                let kw = match inp.as_str() {
                    "global" => Keyword(Global),
                    "vars" => Keyword(Vars),
                    "lists" => Keyword(Lists),
                    "broadcasts" => Keyword(Broadcasts),
                    "costumes" => Keyword(Costumes),
                    "sounds" => Keyword(Sounds),
                    "SVG" => Keyword(Svg),
                    "PNG" => Keyword(Png),
                    "WAV" => Keyword(Wav),
                    "MP4" => Keyword(Mp4),
                    "repeat" => Keyword(Repeat),
                    "if" => Keyword(If),
                    "else" => Keyword(Else),
                    _ => Ident(inp),
                };

                Ok(kw)
            }

            c => Err(LexerError::IllegalChar {
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
        use Keyword::*;

        let input = "global vars lists broadcasts foo costumes if";
        assert_eq!(
            tokenize(input).unwrap(),
            vec![
                Token::Keyword(Global),
                Token::Keyword(Vars),
                Token::Keyword(Lists),
                Token::Keyword(Broadcasts),
                Token::Ident(String::from("foo")),
                Token::Keyword(Costumes),
                Token::Keyword(If),
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
            LexerError::UnterminatedStringLiteral {
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
            LexerError::IllegalChar {
                c: 'ü',
                span: Span::single(SourcePosition::new(1, 1))
            }
        );
    }
}
