/*
 - parser.rs
 */

use crate::lexer::lexer::Token;
use crate::parser::ast::Node;

fn parse(token: Vec<Token>) {
    let mut current = 0;

    fn parse_program(token: &Vec<Token>, current: &mut usize) -> Node {

    }

    fn parse_function_declaration(token: &Vec<Token>, current: &mut usize) -> Node {

    }

    parse_program(&token, &mut current);
}