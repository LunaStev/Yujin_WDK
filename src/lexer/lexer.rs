/*
 - lexer.rs
 */
use nom::branch::*;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::combinator::{map, map_res, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::*;

use std::str;
use std::str::FromStr;
use std::str::Utf8Error;

use crate::lexer::token::*;

macro_rules! syntax {
    (&func_name: ident, &tag_string: literal, &output_token: expr) => {
        fn &func_name<'a> (s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag(&tag_string), |_| &output_token)(s)
        }
    };
}

syntax! {equal_operator, "==", Token::EQUAL}

pub fn lex_operator(input: &[u8]) -> IResult<&[u8] ,Token> {
    alt(
        equal_operator,

    )(input)
}