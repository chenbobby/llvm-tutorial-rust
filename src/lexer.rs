use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordType {
    // Function definition
    Definition,

    // Function prototype
    Extern,

    // Control flow
    If,
    Then,
    Else,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Reserved keywords
    Keyword(KeywordType),

    // Identifiers for variables, functions, and parameters
    Identifier(String),

    // Puncuation
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Semicolon,

    // Values
    Number(f64),

    // Operators
    Operator(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let regex: Regex = Regex::new(concat!(
        r"(?<comment>#.*\n)",
        r"|(?<keyword_definition>def)",
        r"|(?<keyword_extern>extern)",
        r"|(?<keyword_if>if)",
        r"|(?<keyword_then>then)",
        r"|(?<keyword_else>else)",
        r"|(?<identifier>[a-zA-Z][a-zA-Z0-9]*)",
        r"|(?<open_parenthesis>\()",
        r"|(?<close_parenthesis>\))",
        r"|(?<comma>,)",
        r"|(?<semicolon>;)",
        r"|(?<number>(\d+)|(\d*)(\.\d+))",
        r"|(?<operator>[+\-*/<>(==)])",
    ))
    .unwrap();

    let mut tokens: Vec<Token> = vec![];
    for capture_group in regex.captures_iter(&input) {
        if capture_group.name("comment").is_some() {
            continue;
        } else if capture_group.name("keyword_definition").is_some() {
            tokens.push(Token::Keyword(KeywordType::Definition));
        } else if capture_group.name("keyword_extern").is_some() {
            tokens.push(Token::Keyword(KeywordType::Extern));
        } else if capture_group.name("keyword_if").is_some() {
            tokens.push(Token::Keyword(KeywordType::If));
        } else if capture_group.name("keyword_then").is_some() {
            tokens.push(Token::Keyword(KeywordType::Then));
        } else if capture_group.name("keyword_else").is_some() {
            tokens.push(Token::Keyword(KeywordType::Else));
        } else if let Some(identifier) = capture_group.name("identifier") {
            tokens.push(Token::Identifier(identifier.as_str().to_string()));
        } else if capture_group.name("open_parenthesis").is_some() {
            tokens.push(Token::OpenParenthesis);
        } else if capture_group.name("close_parenthesis").is_some() {
            tokens.push(Token::CloseParenthesis);
        } else if capture_group.name("comma").is_some() {
            tokens.push(Token::Comma);
        } else if capture_group.name("semicolon").is_some() {
            tokens.push(Token::Semicolon);
        } else if let Some(number) = capture_group.name("number") {
            tokens.push(Token::Number(number.as_str().parse::<f64>().unwrap()));
        } else if let Some(operator) = capture_group.name("operator") {
            tokens.push(Token::Operator(operator.as_str().to_string()));
        }
    }

    return tokens;
}

#[cfg(test)]
mod test {
    use super::{tokenize, KeywordType, Token};

    #[test]
    fn it_works() {
        let input = "
            # Compute the x'th finbonnaci number.
            def fib(x)
                if x < 3 then
                    1
                else
                    fib(x-1) + fib(x-2);

            # This expression will compute the 40th fibonacci number.
            fib(40);

            # Import standard library functions.
            extern sin(arg);
            extern cos(arg);
            extern atan2(arg1, arg2);

            # Call imported functions.
            atan2(sin(.4), cos(42));
            ";
        let expected = vec![
            // Compute the x'th finbonnaci number.
            Token::Keyword(KeywordType::Definition),
            Token::Identifier("fib".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("x".to_string()),
            Token::CloseParenthesis,
            Token::Keyword(KeywordType::If),
            Token::Identifier("x".to_string()),
            Token::Operator("<".to_string()),
            Token::Number(3.0),
            Token::Keyword(KeywordType::Then),
            Token::Number(1.0),
            Token::Keyword(KeywordType::Else),
            Token::Identifier("fib".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("x".to_string()),
            Token::Operator("-".to_string()),
            Token::Number(1.0),
            Token::CloseParenthesis,
            Token::Operator("+".to_string()),
            Token::Identifier("fib".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("x".to_string()),
            Token::Operator("-".to_string()),
            Token::Number(2.0),
            Token::CloseParenthesis,
            Token::Semicolon,
            // This expression will compute the 40th fibonacci number.
            Token::Identifier("fib".to_string()),
            Token::OpenParenthesis,
            Token::Number(40.0),
            Token::CloseParenthesis,
            Token::Semicolon,
            // Import standard library functions.
            Token::Keyword(KeywordType::Extern),
            Token::Identifier("sin".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("arg".to_string()),
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::Keyword(KeywordType::Extern),
            Token::Identifier("cos".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("arg".to_string()),
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::Keyword(KeywordType::Extern),
            Token::Identifier("atan2".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("arg1".to_string()),
            Token::Comma,
            Token::Identifier("arg2".to_string()),
            Token::CloseParenthesis,
            Token::Semicolon,
            // Call imported functions.
            Token::Identifier("atan2".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("sin".to_string()),
            Token::OpenParenthesis,
            Token::Number(0.4),
            Token::CloseParenthesis,
            Token::Comma,
            Token::Identifier("cos".to_string()),
            Token::OpenParenthesis,
            Token::Number(42.0),
            Token::CloseParenthesis,
            Token::CloseParenthesis,
            Token::Semicolon,
        ];

        let result = tokenize(input);
        result.iter().zip(expected.iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
    }
}
