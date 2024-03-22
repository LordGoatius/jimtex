use std::{char, fs, path::Path};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Dollar,
    Backslash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Period,
    Comma,
    Equals,
    Colon,
    Exponent,
    Subscript,
    Percent,
    Newline,
    Space,
    FormatDollar,
    FormatDoubleDollar,
    OpenInline,
    OpenDisplay,
    CloseInline,
    CloseDisplay,
    OpenCodeInline,
    OpenCodeDisplay,
    CloseCodeInline,
    CloseCodeDisplay,
    Operator(Operator),
    Text(String),
    Number(String),

    RealNumbers,
    NatrualNumbers,
    ComplexNumbers,
    RationalNumbers,
    Integers,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

pub fn lex(input: &Path) {
    let input: String = fs::read_to_string(input).expect("Invalid file");

    let mut token_string: Vec<Token> = vec![];

    let mut text_vec: Vec<char> = vec![];
    let mut num_vec:  Vec<char> = vec![];

    let mut ignore = 0;

    for (i, char) in input.clone().chars().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }
        match char {
            ','  => token_string.push(Token::Comma),
            ':'  => token_string.push(Token::Comma),
            ' '  => token_string.push(Token::Space),
            '.'  => token_string.push(Token::Period),
            '='  => token_string.push(Token::Equals),
            '%'  => token_string.push(Token::Percent),
            '\n' => token_string.push(Token::Newline),
            '^'  => token_string.push(Token::Exponent),
            '_'  => token_string.push(Token::Subscript),
            '('  => token_string.push(Token::LeftParen),
            '{'  => token_string.push(Token::LeftBrace),
            ')'  => token_string.push(Token::RightParen),
            '}'  => token_string.push(Token::RightBrace),
            '['  => token_string.push(Token::LeftBracket),
            ']'  => token_string.push(Token::RightBracket),
            '+'  => token_string.push(Token::Operator(Operator::Add)),
            '-'  => token_string.push(Token::Operator(Operator::Sub)),
            '*'  => token_string.push(Token::Operator(Operator::Mul)),
            '/'  => token_string.push(Token::Operator(Operator::Div)),
            '\\' => {
                match input.clone().chars().nth(i + 1).unwrap_or_default() {
                    // JimTeX/LaTeX/TeX inline/displat
                    '(' => { token_string.push(Token::OpenInline);   ignore += 1; },
                    '[' => { token_string.push(Token::OpenDisplay);  ignore += 1; },
                    ')' => { token_string.push(Token::CloseInline);  ignore += 1; },
                    ']' => { token_string.push(Token::CloseDisplay); ignore += 1; },
                    '$' => {
                        match input.clone().chars().nth(i + 2).unwrap_or_default() {
                            '(' => { token_string.push(Token::OpenCodeInline);   ignore += 1; },
                            '[' => { token_string.push(Token::OpenCodeDisplay);  ignore += 1; },
                            ')' => { token_string.push(Token::CloseCodeInline);  ignore += 1; },
                            ']' => { token_string.push(Token::CloseCodeDisplay); ignore += 1; },
                            '$' => { token_string.push(Token::Dollar); token_string.push(Token::Dollar); ignore += 1; },
                            _   => token_string.push(Token::Dollar),
                        }
                        ignore += 1;
                    },
                    // Basic Number Sets
                    'Z' => { token_string.push(Token::Integers);        ignore += 1; },
                    'R' => { token_string.push(Token::RealNumbers);     ignore += 1; },
                    'Q' => { token_string.push(Token::RationalNumbers); ignore += 1; },
                    'C' => { token_string.push(Token::ComplexNumbers);  ignore += 1; },
                    'N' => { token_string.push(Token::NatrualNumbers);  ignore += 1; },
                    _   => token_string.push(Token::Backslash),
                }
            },
            '$' => {
                match input.clone().chars().nth(i + 1).unwrap_or_default() {
                    '$' => { token_string.push(Token::FormatDoubleDollar); ignore += 1; },
                    _   => token_string.push(Token::FormatDollar),
                }
            }
            char => {
                if char.is_ascii_digit() {
                    num_vec.push(char);
                } else {
                    text_vec.push(char);
                }
                continue;
            }
        }
        if text_vec.len() > 0 {
            let prev: Token = token_string.pop().expect("Should be impossible");
            token_string.push(Token::Text(text_vec.iter().collect::<String>()));
            token_string.push(prev);
            text_vec = vec![];
        }
        if num_vec.len() > 0 {
            let prev: Token = token_string.pop().expect("Should be impossible");
            token_string.push(Token::Number(num_vec.iter().collect::<String>()));
            token_string.push(prev);
            num_vec = vec![];
        }
    }
    eprintln!("{:?}", token_string);
}
