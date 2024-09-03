use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use chumsky::prelude::*;
use scratch_common_types::{AudioType, ImgType, Value};

use crate::{Ident, ParserInput, Span, Spanned, Token};

type VarsHeader = HashMap<Ident, Option<Value>>;
type ListsHeader = HashMap<Ident, Vec<Value>>;
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

fn ident<'tok, 'src: 'tok>(
) -> impl Parser<'tok, ParserInput<'tok, 'src>, Spanned<Ident>, extra::Err<Rich<'tok, Token<'src>, Span>>>
{
    select! {
        Token::Ident(ident) => Ident::new(ident.to_string())
    }
    .labelled("identifier")
    .map_with(|var_name, e| (var_name, e.span()))
}

fn value<'tok, 'src: 'tok>(
) -> impl Parser<'tok, ParserInput<'tok, 'src>, Value, extra::Err<Rich<'tok, Token<'src>, Span>>> {
    select! {
        Token::Number(num) => Value::Number(num),
        Token::String(string) => Value::String(string.to_string()),
    }
    .labelled("value")
}

pub fn vars_header_parser<'tok, 'src: 'tok>(
) -> impl Parser<'tok, ParserInput<'tok, 'src>, VarsHeader, extra::Err<Rich<'tok, Token<'src>, Span>>>
{
    let decl = ident()
        .then(just(Token::Equals).ignore_then(value()).or_not())
        .then_ignore(just(Token::Semicolon));

    just(Token::Vars).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .validate(|decls, _, emitter| {
                let mut vars = HashMap::new();
                for ((id, span), val) in decls {
                    if vars.insert(id.clone(), val).is_some() {
                        emitter.emit(Rich::custom(
                            span,
                            format!("Variable '{}' already exists", id),
                        ));
                    }
                }
                vars
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}

pub fn lists_header_parser<'tok, 'src: 'tok>(
) -> impl Parser<'tok, ParserInput<'tok, 'src>, ListsHeader, extra::Err<Rich<'tok, Token<'src>, Span>>>
{
    let list = value()
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::BracketOpen), just(Token::BracketClose));

    let decl = ident()
        .then(
            just(Token::Equals)
                .ignore_then(list)
                .or_not()
                .map(Option::unwrap_or_default),
        )
        .then_ignore(just(Token::Semicolon));

    just(Token::Lists).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .validate(|decls, _, emitter| {
                let mut lists = HashMap::new();
                for ((id, span), val) in decls {
                    if lists.insert(id.clone(), val).is_some() {
                        emitter.emit(Rich::custom(span, format!("List '{}' already exists", id)));
                    }
                }
                lists
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}

pub fn broadcasts_header_parser<'tok, 'src: 'tok>() -> impl Parser<
    'tok,
    ParserInput<'tok, 'src>,
    BroadcastsHeader,
    extra::Err<Rich<'tok, Token<'src>, Span>>,
> {
    let decl = ident().then_ignore(just(Token::Semicolon));

    just(Token::Broadcasts).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .validate(|decls, _, emitter| {
                let mut broadcasts = HashSet::new();
                for (id, span) in decls {
                    if !broadcasts.insert(id.clone()) {
                        emitter.emit(Rich::custom(
                            span,
                            format!("Broadcast '{id}' declared twice"),
                        ));
                    }
                }
                broadcasts
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}