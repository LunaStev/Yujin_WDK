#ifndef PARSER_H
#define PARSER_H

enum TokenType {
    // 기본 문법
    Comma,
    Lpar, // (
    Rpar, // )
    Lbrack, // [
    Rbrack, // ]
    Lcurly, // {
    Rcurly, // }
    Plus,
    Minus,
    Equal,
    EqEq, // ==
    //Exclam, // !
    NotEq, // !=
    Lesser, // <
    Greater, // >
    LesserEq,
    GreaterEq,
    Star, // *
    Div, // /
    FloatDiv, // /.
    Percent, // %
    Or,
    And, // &
    Fun,
    Float,
    Pow,
    Dot,
    QuotedStr, // "
    Bool,
    Null,
    Var,
    Count,
    ReadIn,
    WriteOut,
    If,
    While,
    Break,
    Continue,
    Else,
    Endline,
    WS, // white space(s)
    UNK,
/* -----=====----- */
    // Grammar Constructs
    // used by the parser, not the lexer
    Program,
    Line,
    Assign,
    Input,
    Output,
    IfLine,
    LoopLine,
    Expr,
    Term,
    Pred,
    BaseExpr,
    Operator,
    CondOp,
    IfCond,
    IfBody,
    OptElse,
    LoopBody,
    Obj,
    Str,
    Num,
    Frac,
    List,
    ListElem,
    ListExpr
};

#endif //PARSER_H
