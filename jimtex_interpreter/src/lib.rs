pub mod lexer;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lexer::{self, lex};

    #[test]
    fn it_works() {
        lex(Path::new("/home/lordgoatius/git/jimtex/jimtex_interpreter/src/test_file.tex"));
    }
}
