use crate::{
    media_type::{AudioType, ImgType},
    span::{SourcePosition, Span},
};

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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Eof => write!(f, "EOF"),
            Token::Comment(_) => write!(f, "comment"),
            Token::MetaComment(_) => write!(f, "meta comment"),
            Token::Keyword(kw) => write!(f, "{kw}"),
            Token::Ident(_) => write!(f, "identifier"),
            Token::Int(_) => write!(f, "integer"),
            Token::Float(_) => write!(f, "float"),
            Token::Str(_) => write!(f, "string"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Equal => write!(f, "="),
            Token::Comma => write!(f, ","),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::Colon => write!(f, ":"),
            Token::DoubleColon => write!(f, "::"),
            Token::Semicolon => write!(f, ";"),
            Token::Arrow => write!(f, "->"),
            Token::ParenL => write!(f, "("),
            Token::ParenR => write!(f, ")"),
            Token::CurlyL => write!(f, "{{"),
            Token::CurlyR => write!(f, "}}"),
            Token::BracketL => write!(f, "["),
            Token::BracketR => write!(f, "]"),
            Token::ChevronL => write!(f, "<"),
            Token::ChevronR => write!(f, ">"),
        }
    }
}

/// A keyword of the language
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    // headers
    Global,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    // media types
    Img(ImgType),
    Audio(AudioType),

    // control flow
    Repeat,
    If,
    Else,
}

impl Keyword {
    pub fn is_header(&self) -> bool {
        use Keyword::*;

        match *self {
            Global | Vars | Lists | Broadcasts | Costumes | Sounds => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Keyword::Global => write!(f, "global"),
            Keyword::Vars => write!(f, "vars"),
            Keyword::Lists => write!(f, "lists"),
            Keyword::Broadcasts => write!(f, "broadcasts"),
            Keyword::Costumes => write!(f, "costumes"),
            Keyword::Sounds => write!(f, "sounds"),
            Keyword::Img(_) => write!(f, "image type"),
            Keyword::Audio(_) => write!(f, "audio type"),
            Keyword::Repeat => write!(f, "repeat"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
        }
    }
}
