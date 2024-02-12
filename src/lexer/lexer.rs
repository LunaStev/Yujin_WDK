/*
 - lexer.rs
 */

/**
 - 토큰
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    /** int */
    INT(i32),

    /** 변수 */
    VAR,
    /** 숫자 변수 */
    COUNT,
    /** 함수 */
    FUN,
    /** 플러스 */
    PLUS,
    /** 마이너스 */
    MINUS,
    /** 별 */
    STAR,
    /** 슬래시 */
    SLASH,
    /** 임포트 */
    IMPORT,

    /** 세미콜론 */
    SEMICOLON,
    /** 콤마 */
    COMMA,

    /** if */
    IF,

}