pub use cursor::Cursor;

use self::token::Token;

mod cursor;
pub mod token;

pub fn lex(source: &str) -> Vec<Token> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = cursor.advance_token();
        let eof = token == Token::Eof;
        tokens.push(token);

        if eof {
            break;
        }
    }

    tokens
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        use Token::*;

        self.eat(is_whitespace);

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
                        MetaComment(self.eat(|c| c != '\n'))
                    }
                    _ => {
                        self.bump();
                        Comment(self.eat(|c| c != '\n'))
                    }
                },
                _ => Slash,
            },

            // one symbol tokens
            '+' => Plus,
            '-' => Minus,
            '=' => Equal,
            ',' => Comma,
            '*' => Asterisk,
            ';' => Semicolon,
            '(' => ParenL,
            ')' => ParenR,
            '{' => CurlyL,
            '}' => CurlyR,
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

            // negatives or arrow
            '-' => match self.first() {
                '>' => {
                    self.bump();
                    Arrow
                }
                _ => todo!(),
            },

            _ => Illegal(first_char.to_string()),
        }
    }
}
