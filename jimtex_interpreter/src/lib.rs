pub mod lexer;
pub mod parser;
pub mod ast;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lexer::lex;
    use crate::parser::parse;

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
}
