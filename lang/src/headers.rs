use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use chumsky::prelude::*;
use scratch_common_types::{AudioType, ImgType, Value};

use crate::{Ident, ParserInput, Span, Token};

type VarsHeader = HashMap<Ident, Option<Value>>;
type ListsHeader = HashMap<Ident, Option<Vec<Value>>>;
type BroadcastsHeader = HashSet<Ident>;
type CostumesHeader = HashMap<Ident, (ImgType, Option<PathBuf>)>;
type SoundsHeader = HashMap<Ident, (AudioType, Option<PathBuf>)>;

pub struct Headers {
    vars: VarsHeader,
    lists: ListsHeader,
    broadcasts: BroadcastsHeader,
    costumes: CostumesHeader,
    current_costume: Option<usize>,
    sounds: SoundsHeader,
}

// TODO: span
pub fn vars_header_parser<'tok, 'src: 'tok>(
) -> impl Parser<'tok, ParserInput<'tok, 'src>, VarsHeader, extra::Err<Rich<'tok, Token<'src>, Span>>>
{
    let ident = select! {
        Token::Ident(ident) => Ident::new(ident.to_string())
    }
    .labelled("identifier");

    let value = select! {
        Token::Number(num) => Value::Number(num),
        Token::String(string) => Value::String(string.to_string()),
    }
    .labelled("value");

    let decl = ident
        .map_with(|var_name, e| (var_name, e.span()))
        .labelled("variable name")
        .then(
            just(Token::Equals)
                .ignore_then(value.labelled("value"))
                .or_not(),
        )
        .then_ignore(just(Token::Semicolon));

    just(Token::Vars).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            // FIXME: weird error message
            .try_map(|decls, _| {
                let mut vars = HashMap::new();
                for ((ident, span), val) in decls {
                    if vars.insert(ident.clone(), val).is_some() {
                        return Err(Rich::custom(
                            span,
                            format!("Variable '{}' already exists", ident),
                        ));
                    }
                }
                Ok(vars)
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}
