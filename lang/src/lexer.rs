use chumsky::prelude::*;

use crate::Span;

// TODO: meta comments
#[derive(Debug, Clone)]
pub enum Token<'src> {
    Number(f64),
    String(&'src str),
    Ident(&'src str),

    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    AngleOpen,
    AngleClose,
    CurlyOpen,
    CurlyClose,

    Comma,
    Colon,
    DoubleColon,
    Semicolon,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Greater,
    Less,

    Set,
    Vars,
    Lists,
    Broadcasts,
    Costumes,
    Sounds,

    Def,
    When,
    If,
    Else,
    Repeat,

    Img(ImgType),
    Audio(AudioType),
}

#[derive(Debug, Clone, Copy)]
pub enum ImgType {
    Svg,
    Png,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioType {
    Wav,
    Mp4,
}

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    // A parser for numbers
    let number = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Number);

    // A parser for strings
    let string = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .to_slice()
        .map(Token::String);

    // A parser for symbols
    let symbol = just("::")
        .to(Token::DoubleColon)
        .or(one_of("()[]<>{},:;+-*/").map(|symbol: char| match symbol {
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            '[' => Token::BracketOpen,
            ']' => Token::BracketClose,
            '<' => Token::AngleOpen,
            '>' => Token::AngleClose,
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            _ => unreachable!("because of one_of()"),
        }));

    // A parser for identifiers and keywords
    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "set" => Token::Set,
        "vars" => Token::Vars,
        "lists" => Token::Lists,
        "broadcasts" => Token::Broadcasts,
        "costumes" => Token::Costumes,
        "sounds" => Token::Sounds,

        "def" => Token::Def,
        "when" => Token::When,
        "if" => Token::If,
        "else" => Token::Else,
        "repeat" => Token::Repeat,

        "greater" => Token::Greater,
        "less" => Token::Less,

        "SVG" => Token::Img(ImgType::Svg),
        "PNG" => Token::Img(ImgType::Png),
        "WAV" => Token::Audio(AudioType::Wav),
        "MP4" => Token::Audio(AudioType::Mp4),

        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = number.or(string).or(symbol).or(ident);

    // ignore comments
    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
