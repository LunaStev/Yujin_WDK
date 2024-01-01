//
// Created by ubuntu on 23. 12. 30.
//

#ifndef YUJIN_WDK_VISITOR_H
#define YUJIN_WDK_VISITOR_H

#include "AST.h"

typedef struct VISITOR_ {
} visitor_T;

visitor_T* init_visitor();

AST_T* visitor_visit(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_variable_definition(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_function_definition(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_variable(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_function_call(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_string(visitor_T* visitor, AST_T* node);

AST_T* visitor_visit_compound(visitor_T* visitor, AST_T* node);

#endif //YUJIN_WDK_VISITOR_H
