/*
 - main.rs

 - 메인 개발 파일
 */

mod lexer;

mod parser;

use lexer::token::*;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..= '9' => {
                let mut number = 0;
                while let Some(&digit) = chars.peek() {
                    if digit.is_digit(10) {
                        number = number * 10 + digit.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                } tokens.push(Token::INT(number));
            } '+' => {
                chars.next();
                tokens.push(Token::PLUS);
            }, '-' => {
                chars.next();
                tokens.push(Token::MINUS);
            }, '*' => {
                chars.next();
                tokens.push(Token::STAR);
            }, '/' => {
                chars.next();
                tokens.push(Token::SLASH);
            }, _ => {
                chars.next();
                ()
            },
        };
    } tokens
}

fn evaluate(tokens: Vec<Token>) -> i32 {
    let mut result = 0;
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::INT(n) => {
                result = n;
                while let Some(op) = tokens.next() {
                    match op { Token::PLUS => {
                            if let Some(Token::INT(n)) = tokens.next() {
                                result += n;
                            }
                        }, Token::MINUS => {
                            if let Some(Token::INT(n)) = tokens.next() {
                                result -= n;
                            }
                        }, Token::STAR => {
                            if let Some(Token::INT(n)) = tokens.next() {
                                result *= n;
                            }
                        }, Token::SLASH => {
                            if let Some(Token::INT(n)) = tokens.next() {
                                result /= n;
                            }
                        }, _ => unreachable!(),
                    }
                }
            }, _ => unreachable!()
        }
    } result
}

fn main() {
    let input = "5 + 4 - 3 * 2 / 1";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    let result = evaluate(tokens);
    println!("Result: {}", result);
}