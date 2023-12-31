#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Illegal(String, &'static str),

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

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Set,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    // control flow
    Repeat,
    If,
    Else,
}
