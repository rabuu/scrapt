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

//fn headers_parser<'tok, 'src: 'tok>(
//) -> impl Parser<'tok, ParserInput<'tok, 'src>, Headers, extra::Err<Rich<'tok, Token<'src>, Span>>>
//{
//
//}
