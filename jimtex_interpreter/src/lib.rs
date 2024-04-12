#![feature(slice_split_once)]
#![allow(dead_code)]

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod parser_ast;
pub mod ast_types;
pub mod interpreter;
pub mod errors;

#[cfg(test)]
mod tests {
    use core::panic;
    use std::collections::HashMap;
    use std::path::Path;

    use crate::ast_types::{FunctionDeclaration, FunctionDefinition, Identifier, Value};
    use crate::interpreter::ProgramInterpreter;
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::parser_ast::parse_to_ast;

    #[test]
    fn test_hm() {
        let tokens = lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/test_file.tex"));
        parse(tokens);

    }

    #[test]
    fn test_small() {
        let tokens = lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/small_test.tex"));
        let tokens = parse(tokens);
        let program = parse_to_ast(tokens);
        let mut interpreter = ProgramInterpreter::default();
        match interpreter.interpret_program(program) {
            Ok(_)      => (),
            Err(error) => {
                println!("{error}");
                panic!()
            }
        }
    }

    #[test]
    fn test_error() {
        let tokens = lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/error_test.tex"));
        let tokens = parse(tokens);
        let program = parse_to_ast(tokens);
        let mut interpreter = ProgramInterpreter::default();
        match interpreter.interpret_program(program) {
            Ok(_)      => (),
            Err(error) => {
                println!("{error}");
            }
        }
    }
    
    #[test]
    fn test_hashing() {
        use crate::lexer::Token;

        let ident = Identifier::GreekLetter(crate::ast::GreekLetters::Alpha);

        let function_dec = FunctionDeclaration { identifier: ident.clone(), domain: Token::Integers, codomain: Token::Integers };
        let function_def = FunctionDefinition  { identifier: ident, arguments: vec![Identifier::TextIdent("a".to_owned())], expression: crate::ast_types::Expression::Value(Box::new(Value::Number(crate::ast_types::Number::Integer(76.into())))) };

        let mut function_declarations: HashMap<Identifier, FunctionDeclaration> = HashMap::new();

        function_declarations.insert(function_dec.identifier.clone(), function_dec);
        if !function_declarations.contains_key(&function_def.identifier) { panic!() }
    }
}
