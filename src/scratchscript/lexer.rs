#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(isize),
    Float(f64),
    Str(String),

    Plus,
    Minus,
    Equal,
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

    Set,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,
}
