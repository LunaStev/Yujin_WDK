// lexer.rs

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    FUN,
    VAR,
    COUNT,
    WHILE,
    IF,
    ELSE,
    SWITCH,
    FOR,
    RETURN,
    Break,
    Char,

    Import,

    Semicolon,  //  ;

    Lbrace,     //  {
    RBrace,     //  }
    Lparen,     //  (
    Rparen,     //  )
    LBracket,   //  [
    RBracket,   //  ]
    Lt,         //  <
    Rt,         //  >

    Plus,       //  +
    Minus,      //  -
    Multi,      //  *
    Splash,     // /
    Equal,      // ==
    Assign      // =
}