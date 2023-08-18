//! Cursor module
//!
//! This module is stolen from Rusts lexer:
//! https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/cursor.rs.html

use std::str::Chars;

const EOF: char = '\0';

/// Peekable iterator over a char sequence
///
/// Next chars can be peeked with `first()` or `second()`,
/// the cursor can be advanced with `bump()` or `eat()`.
#[derive(Debug)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    prev: char,
}

impl<'a> Cursor<'a> {
    /// Contructor for the Cursor
    pub fn new(source: &'a str) -> Cursor<'a> {
        Cursor {
            chars: source.chars(),
            prev: EOF,
        }
    }

    /// Peek previous char
    pub fn prev(&self) -> char {
        self.prev
    }

    /// Peek first char
    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    /// Peek second char
    pub fn second(&self) -> char {
        let mut it = self.chars.clone();
        it.next();
        it.next().unwrap_or(EOF)
    }

    /// Checks if end of source is reached
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Advance to the next char
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    /// Consumes chars while predicate returns true or EOF is reached
    pub fn eat(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
        let mut eaten = String::new();
        while predicate(self.first()) && !self.is_eof() {
            eaten.push(self.bump().unwrap())
        }
        eaten
    }

    /// Works like `eat()` but includes the previously comsumed character as well
    pub fn eat_with_prev(&mut self, predicate: impl FnMut(char) -> bool) -> String {
        let prev = self.prev;
        let mut eaten = self.eat(predicate);
        eaten.insert(0, prev);
        eaten
    }
}
