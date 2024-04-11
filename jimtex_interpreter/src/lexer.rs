use std::{char, fs, path::Path};
use crate::{ast::{BinOps, Conditionals, GreekLetters, Loops, Statements, UnOps}, ast_types::FunctionCall, parser::{Command, NewCommand}};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Tab,
    Colon,
    Comma,
    Space,
    Dollar,
    Equals,
    Period,
    Newline,
    Percent,
    Exponent,
    Backslash,
    LeftBrace,
    LeftParen,
    Subscript,
    Octothorpe,
    OpenInline,
    RightBrace,
    RightParen,
    CloseInline,
    LeftBracket,
    OpenDisplay,
    CloseDisplay,
    FormatDollar,
    RightBracket,
    OpenCodeInline,
    CloseCodeInline,
    NewlineOperator,
    OpenCodeDisplay,
    CloseCodeDisplay,
    EscapedLeftBrace,
    EscapedOctothorpe,
    EscapedRightBrace,
    FormatDoubleDollar,

    Text(String),
    Number(String),
    Operator(Operator),

    RealNumbers,
    NatrualNumbers,
    ComplexNumbers,
    RationalNumbers,
    Integers,

    // PARSER TOKENS ONLY
    CommandStub(String),
    Command(Command),
    NewCommand(NewCommand),

    GreekLetter(GreekLetters),
    BinOp(BinOps),
    UnOps(UnOps),
    Conditional(Conditionals),
    Statement(Statements),
    Loop(Loops),
    RightArrow,
    FunctionCall(FunctionCall),

    Real(f64),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

pub type TokenString = Vec<Token>;

pub fn lex(input: &Path) -> TokenString {
    let input: String = fs::read_to_string(input).expect("Invalid file");

    let mut token_string: Vec<Token> = vec![];
    let input_vec: Vec<char> = input.clone().chars().collect();

    let mut text_vec: Vec<char> = vec![];
    let mut num_vec:  Vec<char> = vec![];

    let mut ignore = 0;

    for (i, char) in input.chars().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }
        match char {
            '\t' => token_string.push(Token::Tab),
            ','  => token_string.push(Token::Comma),
            ':'  => token_string.push(Token::Colon),
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
            '#'  => token_string.push(Token::Octothorpe),
            '\\' => {
                match input_vec.get(i+1).copied().unwrap_or_default() {
                    '\\'=> { token_string.push(Token::NewlineOperator); ignore += 1; }
                    // JimTeX/LaTeX/TeX inline/display
                    '(' => { token_string.push(Token::OpenInline);   ignore += 1; },
                    '[' => { token_string.push(Token::OpenDisplay);  ignore += 1; },
                    ')' => { token_string.push(Token::CloseInline);  ignore += 1; },
                    ']' => { token_string.push(Token::CloseDisplay);  ignore += 1; },
                    '{' => { token_string.push(Token::EscapedLeftBrace); ignore += 1; },
                    '}' => { token_string.push(Token::EscapedRightBrace); ignore += 1; },
                    '#' => { token_string.push(Token::EscapedOctothorpe); ignore += 1; },
                    '$' => {
                        match input_vec.get(i+2).copied().unwrap_or_default() {
                            '(' => { token_string.push(Token::OpenCodeInline);   ignore += 1; },
                            '[' => { token_string.push(Token::OpenCodeDisplay);  ignore += 1; },
                            ')' => { token_string.push(Token::CloseCodeInline);  ignore += 1; },
                            ']' => { token_string.push(Token::CloseCodeDisplay); ignore += 1; },
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
                match input_vec.get(i+1).copied().unwrap_or_default() {
                    '$' => { token_string.push(Token::FormatDoubleDollar); ignore += 1; },
                    _   => token_string.push(Token::FormatDollar),
                }
            }
            char => {
                if char.is_ascii_digit() {
                    num_vec.push(char);
                    if text_vec.len() > 0 {
                        let prev: Token = token_string.pop().expect("Should be impossible");
                        token_string.push(Token::Text(text_vec.iter().collect::<String>()));
                        token_string.push(prev);
                        text_vec = vec![];
                    }
                } else {
                    text_vec.push(char);
                    if num_vec.len() > 0 {
                        let prev: Token = token_string.pop().expect("Should be impossible");
                        token_string.push(Token::Number(num_vec.iter().collect::<String>()));
                        token_string.push(prev);
                        num_vec = vec![];
                    }
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
    token_string
}
