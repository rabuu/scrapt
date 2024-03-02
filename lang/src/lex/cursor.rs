//! Cursor module
//!
//! This module is stolen from Rusts lexer:
//! https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/cursor.rs.html

use std::str::Chars;

use crate::span::{SourcePosition, Span};

const EOF: char = '\0';

/// Peekable iterator over a char sequence
///
/// Next chars can be peeked with `first()` or `second()`,
/// the cursor can be advanced with `bump()` or `eat()`.
#[derive(Debug)]
pub struct Cursor<'a> {
    // TODO: maybe better peeking using Peekable
    chars: Chars<'a>,
    prev: char,

    // span information
    curr_pos: SourcePosition,
    prev_pos: SourcePosition,
}

impl<'a> Cursor<'a> {
    /// Contructor for the Cursor
    pub fn new(source: &'a str) -> Cursor<'a> {
        Cursor {
            chars: source.chars(),
            prev: EOF,
            curr_pos: (1, 1),
            prev_pos: (1, 0),
        }
    }

    /// Peek previous char
    pub fn peek_prev(&self) -> char {
        self.prev
    }

    /// Peek current char
    pub fn peek_this(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    /// Peek next char
    pub fn peek_next(&self) -> char {
        self.chars.clone().skip(1).next().unwrap_or(EOF)
    }

    /// Checks if end of source is reached
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Advance to the next char
    pub fn bump(&mut self) -> Option<(char, SourcePosition)> {
        let c = self.chars.next()?;
        self.prev = c;
        self.prev_pos = self.curr_pos;

        if self.prev == '\n' {
            self.curr_pos.0 += 1;
            self.curr_pos.1 = 1;
        } else {
            self.curr_pos.1 += 1;
        }

        Some((c, self.prev_pos))
    }

    /// Consumes chars while predicate returns true until EOF is reached
    pub fn eat(&mut self, predicate: impl Fn(char) -> bool) -> Option<(String, Span)> {
        let begin = if predicate(self.peek_this()) {
            self.curr_pos
        } else {
            return None;
        };

        let mut eaten = String::new();
        let mut end = None;

        while let Some((c, pos)) = self.bump() {
            if !predicate(c) || self.is_eof() {
                break;
            }

            end = Some(pos);
            eaten.push(c);
        }

        Some((eaten, Span { begin, end }))
    }

    /// Works like `eat()` but includes the previously comsumed character as well
    pub fn eat_with_prev(&mut self, predicate: impl Fn(char) -> bool) -> Option<(String, Span)> {
        let prev = self.prev;
        let begin = self.prev_pos;

        let (mut eaten, mut span) = self.eat(predicate)?;
        eaten.insert(0, prev);
        span.begin = begin;

        Some((eaten, span))
    }
}
