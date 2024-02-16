/*
 - token.rs
 */

use nom::InputLength;

#[derive(Debug, PartialEq)]
pub enum Token {
    INT(i32),

    VAR,
    COUNT,
    FUN,

    PLUS,
    MINUS,
    STAR,
    SLASH,
    GTEQUAL, // Greater Than Equal
    LTEQUAL, // Les Than Equal
    GT, // Greater Than
    LT, // Less Than

    IMPORT,

    NOT,
    RETURN,

    SEMICOLON,
    EQUAL,
    NOTEQUAL,
    COMMA,

    IF,
    ELSE,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tok:&'a [Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Token]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

