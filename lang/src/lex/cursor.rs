//! Cursor module
//!
//! This module is inspired by Rusts lexer but slightly modified.
//! <https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/cursor.rs.html>

use std::str::Chars;

use itertools::{peek_nth, PeekNth};

use crate::span::SourcePosition;

const EOF: char = '\0';

/// Peekable iterator over a char sequence
///
/// Next chars can be peeked with `peek_this()` or `peek_next()`,
/// the cursor can be advanced with `bump()` or `eat()`.
#[derive(Debug)]
pub(super) struct Cursor<'a> {
    chars: PeekNth<Chars<'a>>,
    prev: char,

    curr_pos: SourcePosition,
    prev_pos: SourcePosition,
}

impl<'a> Cursor<'a> {
    /// Contructor for the Cursor
    pub fn new(source: &'a str) -> Cursor<'a> {
        Cursor {
            chars: peek_nth(source.chars()),
            prev: EOF,
            curr_pos: SourcePosition::new(1, 1),
            prev_pos: SourcePosition::new(1, 0),
        }
    }

    /// Peek current char
    ///
    /// Note that a &mut self is needed for peeking
    pub fn peek_this(&mut self) -> char {
        self.chars.peek().copied().unwrap_or(EOF)
    }

    /// Peek next char
    ///
    /// Note that a &mut self is needed for peeking
    pub fn peek_next(&mut self) -> char {
        self.chars.peek_nth(1).copied().unwrap_or(EOF)
    }

    /// Get current position
    pub fn position(&self) -> SourcePosition {
        self.curr_pos
    }

    /// Get previous position
    pub fn prev_position(&self) -> SourcePosition {
        self.prev_pos
    }

    /// Checks if end of source is reached
    pub fn is_eof(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    /// Advance to the next char
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        self.prev_pos = self.curr_pos;

        if self.prev == '\n' {
            self.curr_pos.row += 1;
            self.curr_pos.col = 1;
        } else {
            self.curr_pos.col += 1;
        }

        Some(c)
    }

    /// Consumes chars while predicate returns true until EOF is reached
    pub fn eat(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut eaten = String::new();
        while predicate(self.peek_this()) && !self.is_eof() {
            eaten.push(self.bump().unwrap());
        }

        eaten
    }

    /// Works like `eat()` but includes the previously comsumed character as well
    pub fn eat_with_prev(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let prev = self.prev;

        let mut eaten = self.eat(predicate);
        eaten.insert(0, prev);

        eaten
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_peek_and_bump() {
        let mut cursor = Cursor::new("abc");

        assert_eq!(cursor.peek_this(), 'a');
        assert_eq!(cursor.peek_next(), 'b');

        assert_eq!(cursor.bump(), Some('a'));
        assert_eq!(cursor.bump(), Some('b'));

        assert_eq!(cursor.peek_this(), 'c');
        assert_eq!(cursor.peek_next(), '\0');
    }

    #[test]
    fn cursor_eat() {
        let mut cursor = Cursor::new("foo bar");

        assert_eq!(cursor.eat(|c| !c.is_whitespace()), String::from("foo"));
        assert_eq!(cursor.eat(|c| c.is_whitespace()), String::from(" "));
        assert_eq!(cursor.eat(|c| c.is_whitespace()), String::from(""));
        assert_eq!(cursor.eat(|c| c.is_ascii_alphabetic()), String::from("bar"));
    }

    #[test]
    fn cursor_position() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(cursor.position(), SourcePosition::new(1, 1));
        cursor.bump();
        assert_eq!(cursor.position(), SourcePosition::new(1, 2));
    }
}
