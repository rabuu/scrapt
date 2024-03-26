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

    Global,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    Img(ImgType),
    Audio(AudioType),

    Repeat,
    If,
    Else,
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

    pub fn is_header(&self) -> bool {
        use Token::*;
        matches!(
            *self,
            Global | Vars | Lists | Broadcasts | Costumes | Sounds
        )
    }

    pub fn try_to_inner_string(self) -> Option<String> {
        match self {
            Token::Comment(str) => Some(str),
            Token::MetaComment(str) => Some(str),
            Token::Ident(str) => Some(str),
            Token::Str(str) => Some(str),
            _ => None,
        }
    }

    pub fn try_to_inner_img_type(self) -> Option<ImgType> {
        if let Token::Img(img_type) = self {
            Some(img_type)
        } else {
            None
        }
    }

    pub fn try_to_inner_audio_type(self) -> Option<AudioType> {
        if let Token::Audio(audio_type) = self {
            Some(audio_type)
        } else {
            None
        }
    }

    pub fn kind(&self) -> TokenKind {
        match *self {
            Token::Eof => TokenKind::Eof,
            Token::Comment(_) => TokenKind::Comment,
            Token::MetaComment(_) => TokenKind::MetaComment,
            Token::Ident(_) => TokenKind::Ident,
            Token::Int(_) => TokenKind::Int,
            Token::Float(_) => TokenKind::Float,
            Token::Str(_) => TokenKind::Str,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Equal => TokenKind::Equal,
            Token::Comma => TokenKind::Comma,
            Token::Slash => TokenKind::Slash,
            Token::Asterisk => TokenKind::Asterisk,
            Token::Colon => TokenKind::Colon,
            Token::DoubleColon => TokenKind::DoubleColon,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Arrow => TokenKind::Arrow,
            Token::ParenL => TokenKind::ParenL,
            Token::ParenR => TokenKind::ParenR,
            Token::CurlyL => TokenKind::CurlyL,
            Token::CurlyR => TokenKind::CurlyR,
            Token::BracketL => TokenKind::BracketL,
            Token::BracketR => TokenKind::BracketR,
            Token::ChevronL => TokenKind::ChevronL,
            Token::ChevronR => TokenKind::ChevronR,
            Token::Global => TokenKind::Global,
            Token::Vars => TokenKind::Vars,
            Token::Lists => TokenKind::Lists,
            Token::Broadcasts => TokenKind::Broadcasts,
            Token::Costumes => TokenKind::Costumes,
            Token::Sounds => TokenKind::Sounds,
            Token::Img(_) => TokenKind::ImgType,
            Token::Audio(_) => TokenKind::AudioType,
            Token::Repeat => TokenKind::Repeat,
            Token::If => TokenKind::If,
            Token::Else => TokenKind::Else,
        }
    }
}

/// The kind of a [Token] without the payload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Eof,

    Comment,
    MetaComment,

    Ident,
    Int,
    Float,
    Str,

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

    Global,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    ImgType,
    AudioType,

    Repeat,
    If,
    Else,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::MetaComment => write!(f, "meta comment"),
            TokenKind::Ident => write!(f, "identifier"),
            TokenKind::Int => write!(f, "integer"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Str => write!(f, "string"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::DoubleColon => write!(f, "::"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::ParenL => write!(f, "("),
            TokenKind::ParenR => write!(f, ")"),
            TokenKind::CurlyL => write!(f, "{{"),
            TokenKind::CurlyR => write!(f, "}}"),
            TokenKind::BracketL => write!(f, "["),
            TokenKind::BracketR => write!(f, "]"),
            TokenKind::ChevronL => write!(f, "<"),
            TokenKind::ChevronR => write!(f, ">"),
            TokenKind::Global => write!(f, "global"),
            TokenKind::Vars => write!(f, "vars"),
            TokenKind::Lists => write!(f, "lists"),
            TokenKind::Broadcasts => write!(f, "broadcasts"),
            TokenKind::Costumes => write!(f, "costumes"),
            TokenKind::Sounds => write!(f, "sounds"),
            TokenKind::ImgType => write!(f, "image type"),
            TokenKind::AudioType => write!(f, "audio type"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Repeat => write!(f, "repeat"),
        }
    }
}
