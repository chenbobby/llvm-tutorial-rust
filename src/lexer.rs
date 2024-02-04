use std::any::Any;

use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordType {
    Definition,
    Extern,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(KeywordType),
    Semicolon,
    OpenParenthisis,
    CloseParenthisis,
    Comma,
    Identifier(String),
    Number(f64),
    Operator(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let regex: Regex = Regex::new(concat!(
        r"(?P<comment>#.*\n)",
        r"|(?P<definition>def)",
        r"|(?P<keyword_extern>extern)",
        r"|(?P<semicolon>;)",
        r"|(?P<open_parenthisis>\()",
        r"|(?P<close_parenthisis>\))",
        r"|(?P<comma>,)",
        r"|(?P<identifier>[a-zA-Z][a-zA-Z0-9]*)",
        r"|(?P<number>\d+(\.\d+)?)",
        r"|(?P<operator>[+\-*/])",
    ))
    .unwrap();

    let mut tokens: Vec<Token> = vec![];
    for captures in regex.captures_iter(&input) {
        if captures.name("comment").is_some() {
            continue;
        } else if captures.name("definition").is_some() {
            tokens.push(Token::Keyword(KeywordType::Definition));
        } else if captures.name("keyword_extern").is_some() {
            tokens.push(Token::Keyword(KeywordType::Extern));
        } else if captures.name("semicolon").is_some() {
            tokens.push(Token::Semicolon);
        } else if captures.name("open_parenthisis").is_some() {
            tokens.push(Token::OpenParenthisis);
        } else if captures.name("close_parenthisis").is_some() {
            tokens.push(Token::CloseParenthisis);
        } else if captures.name("comma").is_some() {
            tokens.push(Token::Comma);
        } else if let Some(identifier) = captures.name("identifier") {
            tokens.push(Token::Identifier(identifier.as_str().to_string()));
        } else if let Some(number) = captures.name("number") {
            tokens.push(Token::Number(number.as_str().parse().unwrap()));
        } else if let Some(operator) = captures.name("operator") {
            tokens.push(Token::Operator(operator.as_str().to_string()));
        }
    }

    return tokens;
}
