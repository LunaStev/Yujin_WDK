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
    splash,     // /
    Assign      // =
}