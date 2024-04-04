use crate::lexer::{TokenString, Token};
use crate::ast::*;

// TODO:
// Parse commands options ({},[])

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    required: TokenString,
    opt: TokenString
}

pub enum Environment {
    Center, 
    Tabular,
    Math,
    Array,
    EqnArray,
    Equation,
    Matrix
}

#[derive(Debug, Clone)]
pub struct NewCommand {
    cmd:  Command,
    args: u8,
    def:  TokenString
}

fn make_commands(tokens: TokenString) -> TokenString {
    let mut ret_vec = vec![];
    let input_vec = tokens.clone();

    let mut ignore = 0;
    for (i, token) in tokens.into_iter().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }

        match token {
            Token::Backslash => {
                if let Token::Text(text) = &input_vec[i+1] {
                    ret_vec.push(Token::CommandStub(text.to_string()));
                    ignore += 1;
                } else {
                    ret_vec.push(Token::Backslash);
                }
            }
            catch => ret_vec.push(catch)
        }
    }
    ret_vec.into_iter().map(|token| parse_cmd_stub(token)).collect()
}

fn command_option_parser(tokens: TokenString) -> TokenString {
    let mut ret_vec = vec![];
    let mut ignore = 0;
    let token_ref = tokens.clone();

    for (i, token) in tokens.into_iter().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }

        if let Token::CommandStub(text) = token {

        } else {
            ret_vec.push(token);
        }
    }
    ret_vec
}

fn parse_cmd_stub(token: Token) -> Token {
    match token {
        Token::CommandStub(cmd) => {
            match &cmd[..] {
                // NOTE: All these should not change
                "alpha"      => Token::GreekLetter(GreekLetters::Alpha),
                "beta"       => Token::GreekLetter(GreekLetters::Beta),
                "gamma"      => Token::GreekLetter(GreekLetters::Gamma),
                "delta"      => Token::GreekLetter(GreekLetters::Delta),
                "epsilon"    => Token::GreekLetter(GreekLetters::Epsilon),
                "zeta"       => Token::GreekLetter(GreekLetters::Zeta),
                "theta"      => Token::GreekLetter(GreekLetters::Theta),
                "iota"       => Token::GreekLetter(GreekLetters::Iota),
                "kappa"      => Token::GreekLetter(GreekLetters::Kappa),
                "lambda"     => Token::GreekLetter(GreekLetters::Lambda),
                "nu"         => Token::GreekLetter(GreekLetters::Nu),
                "xi"         => Token::GreekLetter(GreekLetters::Xi),
                "pi"         => Token::GreekLetter(GreekLetters::Pi),
                "rho"        => Token::GreekLetter(GreekLetters::Rho),
                "sigma"      => Token::GreekLetter(GreekLetters::Sigma),
                "tau"        => Token::GreekLetter(GreekLetters::Tau),
                "upsilon"    => Token::GreekLetter(GreekLetters::Upsilon),
                "phi"        => Token::GreekLetter(GreekLetters::Phi),
                "chi"        => Token::GreekLetter(GreekLetters::Chi),
                "psi"        => Token::GreekLetter(GreekLetters::Psi),
                "omega"      => Token::GreekLetter(GreekLetters::Omega),
                "varepsilon" => Token::GreekLetter(GreekLetters::VarEpsilon),
                "vartheta"   => Token::GreekLetter(GreekLetters::VarTheta),
                "varrho"     => Token::GreekLetter(GreekLetters::VarRho),
                "varsigma"   => Token::GreekLetter(GreekLetters::VarSigma),
                "varphi"     => Token::GreekLetter(GreekLetters::VarPhi),
                "Gamma"      => Token::GreekLetter(GreekLetters::UpperGamma),
                "Delta"      => Token::GreekLetter(GreekLetters::UpperDelta),
                "Theta"      => Token::GreekLetter(GreekLetters::UpperTheta),
                "Lambda"     => Token::GreekLetter(GreekLetters::UpperLambda),
                "Xi"         => Token::GreekLetter(GreekLetters::UpperXi),
                "Pi"         => Token::GreekLetter(GreekLetters::UpperPi),
                "Sigma"      => Token::GreekLetter(GreekLetters::UpperSigma),
                "Upsilon"    => Token::GreekLetter(GreekLetters::UpperUpsilon),
                "Phi"        => Token::GreekLetter(GreekLetters::UpperPhi),
                "Psi"        => Token::GreekLetter(GreekLetters::UpperPsi),
                "Omega"      => Token::GreekLetter(GreekLetters::UpperOmega),

                // NOTE: The Following, are currently for LaTeX, not ExTeX,
                // This will need to be modified
                "pm"         => Token::BinOp(BinOps::PlusMinus),
                "setminus"   => Token::BinOp(BinOps::SetDifference),
                // Need tuned
                "cdot"       => Token::BinOp(BinOps::Multiply),
                "times"      => Token::BinOp(BinOps::Multiply),
                "ast"        => Token::BinOp(BinOps::Multiply),
                // Need tuned
                "div"        => Token::BinOp(BinOps::Divide),
                "wedge"      => Token::BinOp(BinOps::BoolAnd),
                "vee"        => Token::BinOp(BinOps::BoolOr),
                "oplus"      => Token::BinOp(BinOps::BoolXor),
                "extprod"    => Token::BinOp(BinOps::ExternalDirectProduct),
                "intprod"    => Token::BinOp(BinOps::InternalDirectProduct),
                "cap"        => Token::BinOp(BinOps::Union),
                "cup"        => Token::BinOp(BinOps::Intersection),

                "equiv"      => Token::Statement(Statements::Equivalent),
                "sim"        => Token::Statement(Statements::Similar),
                "simeq"      => Token::Statement(Statements::SimilarEq),
                "subset"     => Token::Statement(Statements::Subset),
                "supset"     => Token::Statement(Statements::Superset),
                "subseteq"   => Token::Statement(Statements::SubsetEq),
                "supseteq"   => Token::Statement(Statements::SupersetEq),
                "parallel"   => Token::Statement(Statements::Parallel),
                "perp"       => Token::Statement(Statements::Perpendicular),
                "models"     => Token::Statement(Statements::Models),
                "cong"       => Token::Statement(Statements::Congruent),
                "in"         => Token::Statement(Statements::In),
                "ni"         => Token::Statement(Statements::NotIn),

                "forall"     => Token::Loop(Loops::Forall),
                "sum"        => Token::Loop(Loops::Sum),
                "prod"       => Token::Loop(Loops::Product),
                "bigcup"     => Token::Loop(Loops::Union),
                "bigcap"     => Token::Loop(Loops::Intersection),
                "bigwedge"   => Token::Loop(Loops::And),
                "bigvee"     => Token::Loop(Loops::Or),

                "equals"     => Token::Conditional(Conditionals::Equals),
                "approx"     => Token::Conditional(Conditionals::Approx),
                "leq"        => Token::Conditional(Conditionals::LessEq),
                "geq"        => Token::Conditional(Conditionals::GreaterEq),
                "less"       => Token::Conditional(Conditionals::Less),
                "greater"    => Token::Conditional(Conditionals::Greater),
                "ifcong"     => Token::Conditional(Conditionals::Congruent),
                "ifin"       => Token::Conditional(Conditionals::In),
                "ifnin"      => Token::Conditional(Conditionals::NotIn),

                default      => Token::CommandStub(default.to_owned()),
            }
        }
        other => other,
    }
}

pub fn parse(tokens: TokenString) {
    let tokens = make_commands(tokens);
    eprintln!("{tokens:?}");
}
