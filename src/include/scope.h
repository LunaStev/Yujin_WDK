//
// Created by ubuntu on 23. 12. 30.
//

#ifndef YUJIN_WDK_SCOPE_H
#define YUJIN_WDK_SCOPE_H

#include "AST.h"

typedef struct SCOPE {
    AST_T** function_definitions;
    size_t function_definitions_size;

    AST_T** variable_definitions;
    size_t variable_definitions_size;
} scope_T;

scope_T* init_scope();

AST_T* scope_add_function_definition(scope_T* scope, AST_T* fdef);

AST_T* scope_get_function_definition(scope_T* scope, const char* fname);

AST_T* scope_add_variable_definition(scope_T* scope, AST_T* vdef);

AST_T* scope_get_variable_definition(scope_T* scope, const char* name);

#endif //YUJIN_WDK_SCOPE_H
