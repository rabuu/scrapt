use std::fmt;

use chumsky::prelude::*;
use scratch_common_types::{AudioType, ImgType, Number};

use crate::Span;

// TODO: meta comments
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Number(Number),
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
    Equals,

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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::String(s) => write!(f, "\"{s}\""),
            Token::Ident(i) => write!(f, "{i}"),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
            Token::AngleOpen => write!(f, "<"),
            Token::AngleClose => write!(f, ">"),
            Token::CurlyOpen => write!(f, "{{"),
            Token::CurlyClose => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::DoubleColon => write!(f, "::"),
            Token::Semicolon => write!(f, ";"),
            Token::Equals => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Greater => write!(f, "greater"),
            Token::Less => write!(f, "less"),
            Token::Set => write!(f, "set"),
            Token::Vars => write!(f, "vars"),
            Token::Lists => write!(f, "lists"),
            Token::Broadcasts => write!(f, "broadcasts"),
            Token::Costumes => write!(f, "costumes"),
            Token::Sounds => write!(f, "sounds"),
            Token::Def => write!(f, "def"),
            Token::When => write!(f, "when"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Repeat => write!(f, "repeat"),
            Token::Img(x) => write!(f, "{x}"),
            Token::Audio(x) => write!(f, "{x}"),
        }
    }
}

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    // A parser for floats
    let float = text::int(10)
        .then(just('.').then(text::digits(10)))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(|f| Token::Number(Number::Float(f)));

    // A parser for integers
    let integer = text::int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(|i| Token::Number(Number::Integer(i)));

    // A number is either a float or an integer
    let number = float.or(integer);

    // A parser for strings
    let escape = just('\\')
        .then(choice((
            just('\\'),
            just('/'),
            just('"'),
            just('b').to('\x08'),
            just('f').to('\x0C'),
            just('n').to('\n'),
            just('r').to('\r'),
            just('t').to('\t'),
        )))
        .ignored()
        .boxed();

    let string = none_of(r#"\""#)
        .ignored()
        .or(escape)
        .repeated()
        .to_slice()
        .map(Token::String)
        .delimited_by(just('"'), just('"'))
        .boxed();

    // A parser for symbols
    let symbol = just("::")
        .to(Token::DoubleColon)
        .or(one_of("()[]<>{}=,:;+-*/").map(|symbol: char| match symbol {
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            '[' => Token::BracketOpen,
            ']' => Token::BracketClose,
            '<' => Token::AngleOpen,
            '>' => Token::AngleClose,
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            '=' => Token::Equals,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let (tokens, errors) = lexer()
            .parse(r#"vars {hello: MP4 = 1.3; foo=8; [ "hello"] }"#)
            .into_output_errors();

        assert!(errors.is_empty());

        let tokens: Vec<Token> = tokens
            .unwrap()
            .into_iter()
            .map(|(tok, _span)| tok)
            .collect();

        use Token::*;
        assert_eq!(
            tokens,
            vec![
                Vars,
                CurlyOpen,
                Ident("hello"),
                Colon,
                Audio(AudioType::Mp4),
                Equals,
                Number(scratch_common_types::Number::Float(1.3)),
                Semicolon,
                Ident("foo"),
                Equals,
                Number(scratch_common_types::Number::Integer(8)),
                Semicolon,
                BracketOpen,
                String("hello"),
                BracketClose,
                CurlyClose,
            ]
        );
    }
}
