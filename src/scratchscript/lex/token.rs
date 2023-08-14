#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    Comment(String),
    MetaComment(String),

    Ident(String),
    RawIdent(String),
    Int(isize),
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
    ChevronL,
    ChevronR,

    // keywords
    Set,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,
}
