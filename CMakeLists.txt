cmake_minimum_required(VERSION 3.27)
project(Yujin_WDK C)

set(CMAKE_C_STANDARD 11)

add_executable(Yujin_WDK src/lexer.c
        src/lexer.h
        src/semantic.h
        src/semantic.c
        src/parser.h
        src/parser.c)
