use crate::lex;

pub fn parse(source: impl AsRef<str>) {
    let tokens = lex::tokenize(source.as_ref());
}
