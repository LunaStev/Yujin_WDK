mod vm;
mod compiler;
mod core;
mod parser;
mod lexer;
mod token;

#[derive(Debug, PartialEq)]
pub enum CharacterType {
    NewLine,    // \n
}