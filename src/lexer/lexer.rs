/*
 - lexer.rs
 */

#[derive(Debug, PartialEq)]
pub enum Token {
    INT(i32), // int
    VAR, // 변수
    COUNT, // 숫자 변수
    FUN, // 함수
    PLUS, // 플러스
    MINUS, // 마이너스
    STAR, // 별
    SLASH, // 슬래시
    IMPORT, // 임포트
}