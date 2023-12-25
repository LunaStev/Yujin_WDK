#ifndef PARSER_H
#define PARSER_H

typedef enum {
    /*
     * 사칙 연산
     */
    PLU     = '+',
    SUB     = '-',
    MUL     = '*',
    MOD     = '%',
    DIV     = '/',

    /*
     *
     */
    NOT     = '!',

    /*
     *
     */
    LBRACK      = '[',
    RBRACK      = ']',
    LBRACE      = '{',
    RBRACE      = '}',
    LPAREN      = '(',
    RPAREN      = ')',
    GT      = '>',
    LT      = '<',

    /*
     *
     */
    BSLASH      = '//',
};

#endif //PARSER_H
