pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lexer::lex;

    #[test]
    fn test_hm() {
        lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/test_file.tex"));
    }

    #[test]
    fn test_combinations() {
        lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/new_test.tex"))
    }
}
