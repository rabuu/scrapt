/// A lexical Token of the language
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
