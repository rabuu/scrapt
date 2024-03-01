pub use cursor::Cursor;

pub use self::token::{Keyword, Token};

mod cursor;
mod token;

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = cursor.advance_token();
        let is_eof = token == Token::Eof;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    tokens
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        use Token::*;

        self.eat(|c| c.is_whitespace());

        let first_char = match self.bump() {
            Some(c) => c,
            None => return Eof,
        };

        match first_char {
            // slash or comment
            '/' => match self.first() {
                '/' => match self.second() {
                    '/' => {
                        self.bump();
                        self.bump();
                        MetaComment(self.eat(|c| c != '\n').trim().to_string())
                    }
                    _ => {
                        self.bump();
                        Comment(self.eat(|c| c != '\n').trim().to_string())
                    }
                },
                _ => Slash,
            },

            // one symbol tokens
            '+' => Plus,
            '=' => Equal,
            ',' => Comma,
            '*' => Asterisk,
            ';' => Semicolon,
            '(' => ParenL,
            ')' => ParenR,
            '{' => CurlyL,
            '}' => CurlyR,
            '[' => BracketL,
            ']' => BracketR,
            '<' => ChevronL,
            '>' => ChevronR,

            // colons
            ':' => match self.first() {
                ':' => {
                    self.bump();
                    DoubleColon
                }
                _ => Colon,
            },

            // minus or arrow
            '-' => match self.first() {
                '>' => {
                    self.bump();
                    Arrow
                }
                _ => Minus,
            },

            // strings
            '"' => {
                let string = self.eat(|c| c != '"');
                let _closing_del = self.bump();
                Str(string)
            }

            // raw idents
            'r' if self.first() == '#' && self.second().is_ascii_alphabetic() => {
                self.bump();
                Ident(self.eat(|c| c.is_ascii_alphanumeric()))
            }

            // numerics
            c if c.is_ascii_digit() => {
                let inp = self.eat_with_prev(|c| c.is_ascii_digit() || c == '.');

                if let Ok(int) = inp.parse::<usize>() {
                    Int(int)
                } else if let Ok(float) = inp.parse::<f64>() {
                    Float(float)
                } else {
                    Illegal(inp, "Non-number starting with numerical character")
                }
            }

            // keywords and idents
            c if c.is_ascii_alphabetic() => {
                use self::token::Keyword::*;

                let inp = self.eat_with_prev(|c| c.is_ascii_alphanumeric());

                match inp.as_str() {
                    "set" => return Keyword(Set),
                    "vars" => return Keyword(Vars),
                    "lists" => return Keyword(Lists),
                    "broadcasts" => return Keyword(Broadcasts),
                    "costumes" => return Keyword(Costumes),
                    "sounds" => return Keyword(Sounds),
                    "repeat" => return Keyword(Repeat),
                    "if" => return Keyword(If),
                    "else" => return Keyword(Else),
                    _ => (),
                }

                Ident(inp)
            }

            c => Illegal(c.to_string(), "Illegal character"),
        }
    }
}
