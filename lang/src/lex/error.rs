use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexerError {
    #[error("unterminated string literal")]
    UnterminatedStringLiteral,
    #[error("illegal identifier: `{0}`")]
    IllegalIdent(String),
    #[error("non-number starting with numerical character: `{0}`")]
    BeginsWithNumber(String),
    #[error("illegal character: `{0}`")]
    IllegalChar(char),
}
