use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;

use chumsky::input::ValueInput;
use chumsky::prelude::*;

use scratch_sb3::Value;

use super::lexer::Token;
use super::{Ident, ParseErr, Span, Spanned};
use crate::media_types::{AudioType, ImgType};

type SetHeader = HashMap<Ident, Value>;
type VarsHeader = HashMap<Ident, Option<Value>>;
type ListsHeader = HashMap<Ident, Vec<Value>>;
type BroadcastsHeader = HashSet<Ident>;
type CostumesHeader = HashMap<Ident, (ImgType, Option<PathBuf>)>;
type SoundsHeader = HashMap<Ident, (AudioType, Option<PathBuf>)>;

#[derive(Debug)]
pub struct Headers {
    pub set: SetHeader,
    pub vars: VarsHeader,
    pub lists: ListsHeader,
    pub broadcasts: BroadcastsHeader,
    pub costumes: CostumesHeader,
    pub current_costume: Option<usize>,
    pub sounds: SoundsHeader,
}

impl Headers {
    // TODO: better validation & error msg
    pub fn parser<'src, I>() -> impl Parser<'src, I, Headers, ParseErr<'src>>
    where
        I: ValueInput<'src, Token = Token<'src>, Span = Span>,
    {
        any_header()
            .repeated()
            .collect::<Vec<_>>()
            .validate(|headers, e, emitter| {
                let mut set = None;
                let mut vars = None;
                let mut lists = None;
                let mut broadcasts = None;
                let mut costumes = None;
                let mut sounds = None;

                for header in headers {
                    match header {
                        Header::Set(s) if set.is_none() => set = Some(s),
                        Header::Vars(v) if vars.is_none() => vars = Some(v),
                        Header::Lists(l) if lists.is_none() => lists = Some(l),
                        Header::Broadcasts(b) if broadcasts.is_none() => broadcasts = Some(b),
                        Header::Costumes(c) if costumes.is_none() => costumes = Some(c),
                        Header::Sounds(s) if sounds.is_none() => sounds = Some(s),
                        _ => emitter.emit(Rich::custom(
                            e.span(),
                            format!("Duplicate '{}' header", header.kind()),
                        )),
                    }
                }

                let (costumes, current_costume) = costumes.unwrap_or_default();

                Headers {
                    set: set.unwrap_or_default(),
                    vars: vars.unwrap_or_default(),
                    lists: lists.unwrap_or_default(),
                    broadcasts: broadcasts.unwrap_or_default(),
                    costumes,
                    current_costume,
                    sounds: sounds.unwrap_or_default(),
                }
            })
    }
}

enum Header {
    Set(SetHeader),
    Vars(VarsHeader),
    Lists(ListsHeader),
    Broadcasts(BroadcastsHeader),
    Costumes((CostumesHeader, Option<usize>)),
    Sounds(SoundsHeader),
}

impl Header {
    fn kind(&self) -> &'static str {
        match self {
            Header::Set(_) => "set",
            Header::Vars(_) => "vars",
            Header::Lists(_) => "lists",
            Header::Broadcasts(_) => "broadcasts",
            Header::Costumes(_) => "costumes",
            Header::Sounds(_) => "sounds",
        }
    }
}

fn any_header<'src, I>() -> impl Parser<'src, I, Header, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    choice((
        set_header().map(Header::Set),
        vars_header().map(Header::Vars),
        lists_header().map(Header::Lists),
        brodcasts_header().map(Header::Broadcasts),
        costumes_header().map(Header::Costumes),
        sounds_header().map(Header::Sounds),
    ))
}

fn ident<'src, I>() -> impl Parser<'src, I, Spanned<Ident>, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    select! {
        Token::Ident(ident) => Ident::new(ident.to_string())
    }
    .labelled("identifier")
    .map_with(|var_name, e| (var_name, e.span()))
}

fn value<'src, I>() -> impl Parser<'src, I, Value, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    select! {
        Token::Number(num) => Value::Number(num),
        Token::String(string) => Value::String(string.to_string()),
    }
    .labelled("value")
}

// TODO: better validation (on values)
fn set_header<'src, I>() -> impl Parser<'src, I, SetHeader, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    let valid_setting = ident().validate(|(id, span), _, emitter| {
        if !matches!(
            id.as_str(),
            "tempo" | "volume" | "videoTransparency" | "videoState"
        ) {
            emitter.emit(Rich::custom(span, format!("'{id}' is no valid setting")));
        }
        (id, span)
    });

    let decl = valid_setting
        .then(just(Token::Equals).ignore_then(value()))
        .then_ignore(just(Token::Semicolon));

    just(Token::Set).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .validate(|decls, _, emitter| {
                let mut settings = HashMap::new();
                for ((id, span), val) in decls {
                    if settings.insert(id.clone(), val).is_some() {
                        emitter.emit(Rich::custom(
                            span,
                            format!("Variable '{}' already exists", id),
                        ));
                    }
                }
                settings
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}

fn vars_header<'src, I>() -> impl Parser<'src, I, VarsHeader, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
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

fn lists_header<'src, I>() -> impl Parser<'src, I, ListsHeader, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
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

fn brodcasts_header<'src, I>() -> impl Parser<'src, I, BroadcastsHeader, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
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

fn costumes_header<'src, I>()
-> impl Parser<'src, I, (CostumesHeader, Option<usize>), ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    let img_type = select! { Token::Img(img) => img }.labelled("image type");

    let path = select! {
        Token::String(path) => path
    }
    .try_map(|p: &str, span| PathBuf::from_str(p).map_err(|e| Rich::custom(span, e)))
    .labelled("image path");

    let decl = just(Token::Asterisk)
        .or_not()
        .map(|star| star.is_some())
        .then(ident())
        .then(just(Token::Colon).ignore_then(img_type))
        .then(just(Token::Equals).ignore_then(path).or_not())
        .then_ignore(just(Token::Semicolon));

    just(Token::Costumes).ignore_then(
        decl.repeated()
            .at_least(1)
            .enumerate()
            .collect::<Vec<(usize, _)>>()
            .validate(|decls, _, emitter| {
                let mut costumes = HashMap::new();
                let mut current_costume = None;
                for (i, (((star, (id, span)), file_type), path)) in decls {
                    if star {
                        if current_costume.is_some() {
                            emitter.emit(Rich::custom(
                                span,
                                "Cannot mark two costumes as current".to_string(),
                            ))
                        }
                        current_costume = Some(i);
                    }

                    if costumes.insert(id.clone(), (file_type, path)).is_some() {
                        emitter.emit(Rich::custom(
                            span,
                            format!("Broadcast '{id}' declared twice"),
                        ));
                    }
                }

                (costumes, current_costume)
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}

fn sounds_header<'src, I>() -> impl Parser<'src, I, SoundsHeader, ParseErr<'src>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    let audio_type = select! { Token::Audio(audio) => audio }.labelled("audio type");

    let path = select! {
        Token::String(path) => path
    }
    .try_map(|p: &str, span| PathBuf::from_str(p).map_err(|e| Rich::custom(span, e)))
    .labelled("audio path");

    let decl = ident()
        .then(just(Token::Colon).ignore_then(audio_type))
        .then(just(Token::Equals).ignore_then(path).or_not())
        .then_ignore(just(Token::Semicolon));

    just(Token::Sounds).ignore_then(
        decl.repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .validate(|decls, _, emitter| {
                let mut sounds = HashMap::new();
                for (((id, span), file_type), path) in decls {
                    if sounds.insert(id.clone(), (file_type, path)).is_some() {
                        emitter.emit(Rich::custom(
                            span,
                            format!("Broadcast '{id}' declared twice"),
                        ));
                    }
                }
                sounds
            })
            .delimited_by(just(Token::CurlyOpen), just(Token::CurlyClose)),
    )
}
