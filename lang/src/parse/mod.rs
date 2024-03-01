use crate::lex::{Keyword, Token};

pub fn parse(mut tokens: Vec<Token>) -> ParsedTarget {
    let mut target = ParsedTarget::default();

    tokens.reverse();
    while let Some(tok) = tokens.pop() {
        match tok {
            Token::Keyword(Keyword::Costumes) => {
                if tokens.pop() != Some(Token::CurlyL) {
                    panic!("expected open curly");
                }

                while let Some(tok) = tokens.pop() {
                    let name = match tok {
                        Token::Ident(name) => name,
                        err => panic!("expected name, got {err:?}"),
                    };

                    if tokens.pop() != Some(Token::Colon) {
                        panic!("expected colon");
                    }

                    let costume_type = match tokens.pop() {
                        Some(Token::Keyword(kw)) => match kw {
                            Keyword::Svg => CostumeType::Svg,
                            Keyword::Png => CostumeType::Png,
                            _ => panic!("expected SVG or PNG"),
                        },
                        _ => panic!("expected SVG or PNG"),
                    };

                    let path = if let Some(Token::Equal) = tokens.last() {
                        tokens.pop();
                        match tokens.pop() {
                            Some(Token::Str(path)) => Some(path),
                            _ => panic!("expected path"),
                        }
                    } else {
                        None
                    };

                    if tokens.pop() != Some(Token::Semicolon) {
                        panic!("expected semicolon");
                    }

                    target.costumes.push((name.clone(), costume_type, path));

                    if tokens.last() == Some(&Token::CurlyR) {
                        tokens.pop();
                        break;
                    }
                }
            }
            Token::Eof => break,
            tok => todo!("{:?}", tok),
        }
    }

    target
}

type Name = String;
type AssetPath = String;

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(isize),
    Float(f32),
}

#[derive(Debug)]
pub enum CostumeType {
    Svg,
    Png,
}

#[derive(Debug)]
pub enum SoundType {
    Wav,
    Mp4,
}

#[derive(Debug, Default)]
pub struct ParsedTarget {
    global: Vec<(Name, Value)>,
    vars: Vec<(Name, Option<Value>)>,
    lists: Vec<(Name, Option<Vec<Value>>)>,
    broadcasts: Vec<Name>,
    costumes: Vec<(Name, CostumeType, Option<AssetPath>)>,
    sounds: Vec<(Name, SoundType, Option<AssetPath>)>,
}
