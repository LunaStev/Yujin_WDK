//
// Created by ubuntu on 23. 12. 30.
//

#include "include/token.h"
#include <stdlib.h>

token_T* init_token(int type, char* value) {
    token_T* token = calloc(1, sizeof(struct TOKEN));
    token -> type = type;
    token -> value = value;

    return token;
}