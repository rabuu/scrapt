//! Cursor module
//!
//! This module is stolen from Rusts lexer:
//! https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/cursor.rs.html

use std::str::Chars;

use crate::span::SourcePosition;

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

    /// Peek current char
    pub fn peek_this(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    /// Peek next char
    pub fn peek_next(&self) -> char {
        self.chars.clone().skip(1).next().unwrap_or(EOF)
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
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Advance to the next char
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        self.prev_pos = self.curr_pos;

        if self.prev == '\n' {
            self.curr_pos.0 += 1;
            self.curr_pos.1 = 1;
        } else {
            self.curr_pos.1 += 1;
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
        assert_eq!(cursor.peek_this(), ' ');
        assert_eq!(cursor.eat(|c| c.is_whitespace()), String::from(" "));
        assert_eq!(cursor.eat(|c| c.is_whitespace()), String::from(""));
        assert_eq!(cursor.eat(|c| c.is_ascii_alphabetic()), String::from("bar"));
    }

    #[test]
    fn cursor_position() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(cursor.position(), (1, 1));
        cursor.bump();
        assert_eq!(cursor.position(), (1, 2));
    }
}
