//
// Created by ubuntu on 23. 12. 30.
//

#ifndef YUJIN_WDK_TOKEN_H
#define YUJIN_WDK_TOKEN_H

typedef struct TOKEN {
    enum {
        ID,
        EQUALS,
        STRING,
        SEMI,
        LPAREN,
        RPAREN,
        RBRACE,
        LBRACE.
        COMMA,
        EOF
    }; type;
    char* value;
} token_T;

token_T* init_token(int type, char* value);

#endif //YUJIN_WDK_TOKEN_H
