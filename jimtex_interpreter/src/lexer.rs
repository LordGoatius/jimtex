use std::{char, fs, path::Path};

#[derive(Debug)]
pub enum Tokens {
    Dollar,
    Backslash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Number,
    Period,
    Comma,
    Equals,
    Colon,
    Exponent,
    Subscript,
    Percent,
    Newline,
    Space,
    Operator(Operator),
    Text(String)
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

pub fn lex(input: &Path) {
    let input: String = fs::read_to_string(input).expect("Invalid file");

    let mut token_string: Vec<Tokens> = vec![];

    let mut text_vec: Vec<char> = vec![];

    for char in input.chars() {
        match char {
            '$'  => token_string.push(Tokens::Dollar),
            '\\' => token_string.push(Tokens::Backslash),
            '('  => token_string.push(Tokens::LeftParen),
            ')'  => token_string.push(Tokens::RightParen),
            '{'  => token_string.push(Tokens::LeftBrace),
            '}'  => token_string.push(Tokens::RightBrace),
            '['  => token_string.push(Tokens::LeftBracket),
            ']'  => token_string.push(Tokens::RightBracket),
            ','  => token_string.push(Tokens::Comma),
            '.'  => token_string.push(Tokens::Period),
            '='  => token_string.push(Tokens::Equals),
            ':'  => token_string.push(Tokens::Comma),
            '^'  => token_string.push(Tokens::Exponent),
            '_'  => token_string.push(Tokens::Subscript),
            '%'  => token_string.push(Tokens::Percent),
            '\n' => token_string.push(Tokens::Newline),
            ' '  => token_string.push(Tokens::Space),
            '+'  => token_string.push(Tokens::Operator(Operator::Add)),
            '-'  => token_string.push(Tokens::Operator(Operator::Sub)),
            '*'  => token_string.push(Tokens::Operator(Operator::Mul)),
            '/'  => token_string.push(Tokens::Operator(Operator::Div)),
            char => {
                text_vec.push(char);
                continue;
            }
        }
        if text_vec.len() > 0 {
            token_string.push(Tokens::Text(text_vec.iter().collect::<String>()));
            text_vec = vec![];
        }
    }

    eprintln!("{:?}", token_string);
}
