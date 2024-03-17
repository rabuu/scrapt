pub use cursor::Cursor;

use crate::span::Span;

pub use self::token::{Keyword, Token};

mod cursor;
mod token;

pub fn tokenize(source: &str) -> Vec<(Token, Span)> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = cursor.advance_token();
        let is_eof = token.0 == Token::Eof;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    tokens
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> (Token, Span) {
        use Token::*;

        self.eat(|c| c.is_whitespace());

        let begin = self.position();
        let first_char = match self.bump() {
            Some(c) => c,
            None => return (Eof, Span::single(self.position())),
        };

        match first_char {
            // slash or comment
            '/' => match self.this() {
                '/' => match self.next() {
                    '/' => {
                        self.bump();
                        self.bump();
                        (
                            MetaComment(self.eat(|c| c != '\n').trim().to_string()),
                            Span::range(begin, self.prev_position()),
                        )
                    }
                    _ => {
                        self.bump();
                        (
                            Comment(self.eat(|c| c != '\n').trim().to_string()),
                            Span::range(begin, self.prev_position()),
                        )
                    }
                },
                _ => (Slash, Span::single(begin)),
            },

            // one symbol tokens
            '+' => (Plus, Span::single(begin)),
            '=' => (Equal, Span::single(begin)),
            ',' => (Comma, Span::single(begin)),
            '*' => (Asterisk, Span::single(begin)),
            ';' => (Semicolon, Span::single(begin)),
            '(' => (ParenL, Span::single(begin)),
            ')' => (ParenR, Span::single(begin)),
            '{' => (CurlyL, Span::single(begin)),
            '}' => (CurlyR, Span::single(begin)),
            '[' => (BracketL, Span::single(begin)),
            ']' => (BracketR, Span::single(begin)),
            '<' => (ChevronL, Span::single(begin)),
            '>' => (ChevronR, Span::single(begin)),

            // colons
            ':' => match self.this() {
                ':' => {
                    self.bump();
                    (DoubleColon, Span::range(begin, self.prev_position()))
                }
                _ => (Colon, Span::single(begin)),
            },

            // minus or arrow
            '-' => match self.this() {
                '>' => {
                    self.bump();
                    (Arrow, Span::range(begin, self.prev_position()))
                }
                _ => (Minus, Span::single(begin)),
            },

            // strings
            '"' => {
                let string = self.eat(|c| c != '"');
                let closing_del = self.bump();

                if closing_del != Some('"') {
                    return (
                        Illegal(string, "Unterminated string literal"),
                        Span::range(begin, self.prev_position()),
                    );
                }

                (Str(string), Span::range(begin, self.prev_position()))
            }

            // raw idents
            'r' if self.this() == '#' && self.next().is_ascii_alphabetic() => {
                self.bump();
                let ident = self.eat(|c| c.is_ascii_alphanumeric());

                if ident.is_empty() {
                    return (
                        Illegal(ident, "Illegal identifier"),
                        Span::range(begin, self.prev_position()),
                    );
                }

                (Ident(ident), Span::range(begin, self.prev_position()))
            }

            // numerics
            c if c.is_ascii_digit() => {
                let inp = self.eat_with_prev(|c| c.is_ascii_digit() || c == '.');

                if let Ok(int) = inp.parse::<usize>() {
                    (Int(int), Span::range(begin, self.prev_position()))
                } else if let Ok(float) = inp.parse::<f64>() {
                    (Float(float), Span::range(begin, self.prev_position()))
                } else {
                    (
                        Illegal(inp, "Non-number starting with numerical character"),
                        Span::range(begin, self.prev_position()),
                    )
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

                (kw, Span::range(begin, self.prev_position()))
            }

            c => (
                Illegal(c.to_string(), "Illegal character"),
                Span::range(begin, self.prev_position()),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokenization() {
        use Token::*;

        let input = "+-=,/*:;::->(){}[]<>";
        assert_eq!(
            tokenize(input)
                .into_iter()
                .map(|(tok, _)| tok)
                .collect::<Vec<Token>>(),
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
            tokenize(input)
                .into_iter()
                .map(|(tok, _)| tok)
                .collect::<Vec<Token>>(),
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
    fn illegal_character() {
        let input = "ü";
        let tokens = tokenize(input);

        assert_eq!(
            tokens[0].0,
            Token::Illegal("ü".to_string(), "Illegal character")
        );
    }

    #[test]
    fn simple_spanned_tokens() {
        let inp = "* global";
        let tokens = tokenize(inp);

        assert_eq!(
            tokens,
            vec![
                (Token::Asterisk, Span::single((1, 1))),
                (Token::Keyword(Keyword::Global), Span::range((1, 3), (1, 8))),
                (Token::Eof, Span::single((1, 9))),
            ]
        )
    }
}
