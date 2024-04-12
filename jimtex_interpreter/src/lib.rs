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
    use std::path::Path;

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
    fn test_recursion() {
        let tokens = lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/test_recursion.tex"));
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
}
