use crate::span::{SourcePosition, Span};

/// A lexical token of the language including span
#[derive(Debug)]
pub struct SpannedToken {
    pub inner: Token,
    pub span: Span,
}

/// SpannedTokens are equal to a [Token] if the inner token is the same
impl PartialEq<Token> for SpannedToken {
    fn eq(&self, other: &Token) -> bool {
        self.inner == *other
    }
}

/// A lexical token of the language
#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,

    Comment(String),
    MetaComment(String),

    Keyword(Keyword),
    Ident(String),
    Int(usize),
    Float(f64),
    Str(String),

    Plus,
    Minus,
    Equal,
    Comma,
    Slash,
    Asterisk,
    Colon,
    DoubleColon,
    Semicolon,
    Arrow,

    ParenL,
    ParenR,
    CurlyL,
    CurlyR,
    BracketL,
    BracketR,
    ChevronL,
    ChevronR,
}

impl Token {
    pub fn span(self, begin: SourcePosition, end: SourcePosition) -> SpannedToken {
        SpannedToken {
            inner: self,
            span: Span::range(begin, end),
        }
    }

    pub fn position(self, pos: SourcePosition) -> SpannedToken {
        SpannedToken {
            inner: self,
            span: Span::single(pos),
        }
    }
}

/// A keyword of the language
#[derive(Debug, PartialEq)]
pub enum Keyword {
    Global,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    // media types
    Svg,
    Png,
    Wav,
    Mp4,

    // control flow
    Repeat,
    If,
    Else,
}
