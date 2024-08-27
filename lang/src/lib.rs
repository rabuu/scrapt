use chumsky::span::SimpleSpan;

mod lexer;

pub use lexer::lexer;
pub use lexer::Token;

type Span = SimpleSpan<usize>;
