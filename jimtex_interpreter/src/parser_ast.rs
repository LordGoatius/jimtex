use crate::lexer::{TokenString, Token};
use crate::ast::*;
use crate::ast_types::*;

pub fn parse_to_ast(tokens: TokenString) -> Program {
    let mut program: Program = Program { program: vec![] };

    for slice in into_slices(tokens) {
        match slice_type(&slice) {
           SliceType::Expression         => program.push(Statement::Value(parse_expression(slice))),
           SliceType::Declaration        => program.push(Statement::Value(parse_declaration(slice))),
           SliceType::FunctionDefinition => program.push(Statement::Value(parse_function_def(slice))),
        }
        println!("{:#?}", slice);
    }

    program
}

fn parse_expression(tokens: TokenString) -> Statement {

}

fn parse_declaration(tokens: TokenString) -> Statement {
     
}

fn parse_function_def(tokens: TokenString) -> Statement {
     
}

enum SliceType {
    Expression,
    Declaration,
    FunctionDefinition,
}

fn slice_type(tokens: &TokenString) -> SliceType {
    if tokens.contains(&Token::Colon) { SliceType::FunctionDefinition }
    else if tokens.contains(&Token::Equals) { SliceType::Declaration }
    else { SliceType::Expression }
}

fn into_slices(tokens: TokenString) -> Vec<Vec<Token>> {
    let mut slices: Vec<Vec<Token>> = vec![];

    let mut curr_slice: Vec<Token> = vec![];

    let mut balanced = 0;

    for token in tokens {
        match token {
            Token::Comma => {
                if balanced == 0 {
                    slices.push(curr_slice);
                    curr_slice = vec![];
                } else {
                    curr_slice.push(Token::Comma);
                }
            },
            Token::LeftBrace  => balanced += 1,
            Token::RightBrace => balanced -= 1,
            token => curr_slice.push(token),
        }
    }

    slices.push(curr_slice);

    slices
}
