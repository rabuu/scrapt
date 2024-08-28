use chumsky::span::SimpleSpan;

mod headers;
mod lexer;

pub use lexer::lexer;
pub use lexer::Token;

type Ident = String;
type Span = SimpleSpan<usize>;

type Spanned<T> = (T, Span);
type ParserInput<'tok, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tok [(Token<'src>, Span)]>;
