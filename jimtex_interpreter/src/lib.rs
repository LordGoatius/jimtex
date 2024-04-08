pub mod lexer;
pub mod parser;
pub mod ast;
pub mod parser_ast;
pub mod ast_types;

#[cfg(test)]
mod tests {
    use std::path::Path;

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
        parse(tokens);
    }

    #[test]
    fn test_comma_separated() {
        let tokens = lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/small_test.tex"));
        let tokens = parse(tokens);
        let tokens = parse_to_ast(tokens);
        println!("{tokens:?}");
    }
}
