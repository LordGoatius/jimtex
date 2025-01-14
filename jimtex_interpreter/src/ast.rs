use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GreekLetters {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
    Iota,
    Kappa,
    Lambda,
    Mu,
    Nu,
    Xi,
    Pi,
    Rho,
    Sigma,
    Tau,
    Upsilon,
    Phi,
    Chi,
    Psi,
    Omega,

    VarEpsilon,
    VarTheta,
    VarRho,
    VarSigma,
    VarPhi,

    UpperGamma,
    UpperDelta,
    UpperTheta,
    UpperLambda,
    UpperXi,
    UpperPi,
    UpperSigma,
    UpperUpsilon,
    UpperPhi,
    UpperPsi,
    UpperOmega
    // TODO Add more
}

impl Display for GreekLetters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreekLetters::Alpha => write!(f, "\\alpha"),
            GreekLetters::Beta => write!(f, "\\beta"),
            GreekLetters::Gamma => write!(f, "\\gamma"),
            GreekLetters::Delta => write!(f, "\\delta"),
            GreekLetters::Epsilon => write!(f, "\\epsilon"),
            GreekLetters::Zeta => write!(f, "\\zeta"),
            GreekLetters::Eta => write!(f, "\\eta"),
            GreekLetters::Theta => write!(f, "\\theta"),
            GreekLetters::Iota => write!(f, "\\iota"),
            GreekLetters::Kappa => write!(f, "\\kappa"),
            GreekLetters::Lambda => write!(f, "\\lambda"),
            GreekLetters::Mu => write!(f, "\\mu"),
            GreekLetters::Nu => write!(f, "\\nu"),
            GreekLetters::Xi => write!(f, "\\xi"),
            GreekLetters::Pi => write!(f, "\\pi"),
            GreekLetters::Rho => write!(f, "\\rho"),
            GreekLetters::Sigma => write!(f, "\\sigma"),
            GreekLetters::Tau => write!(f, "\\tau"),
            GreekLetters::Upsilon => write!(f, "\\upsilon"),
            GreekLetters::Phi => write!(f, "\\phi"),
            GreekLetters::Chi => write!(f, "\\chi"),
            GreekLetters::Psi => write!(f, "\\psi"),
            GreekLetters::Omega => write!(f, "\\omega"),
            GreekLetters::VarEpsilon => write!(f, "\\varEpsilon"),
            GreekLetters::VarTheta => write!(f, "\\varTheta"),
            GreekLetters::VarRho => write!(f, "\\varRho"),
            GreekLetters::VarSigma => write!(f, "\\varSigma"),
            GreekLetters::VarPhi => write!(f, "\\varPhi"),
            GreekLetters::UpperGamma => write!(f, "\\upperGamma"),
            GreekLetters::UpperDelta => write!(f, "\\upperDelta"),
            GreekLetters::UpperTheta => write!(f, "\\upperTheta"),
            GreekLetters::UpperLambda => write!(f, "\\upperLambda"),
            GreekLetters::UpperXi => write!(f, "\\upperXi"),
            GreekLetters::UpperPi => write!(f, "\\upperPi"),
            GreekLetters::UpperSigma => write!(f, "\\upperSigma"),
            GreekLetters::UpperUpsilon => write!(f, "\\upperUpsilon"),
            GreekLetters::UpperPhi => write!(f, "\\upperPhi"),
            GreekLetters::UpperPsi => write!(f, "\\upperPsi"),
            GreekLetters::UpperOmega => write!(f, "\\upperOmeg"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOps {
    Addition,
    Subtraction,
    PlusMinus,
    SetDifference,
    Multiply,
    Divide,
    BoolAnd,
    BoolOr,
    BoolXor,
    ExternalDirectProduct,
    InternalDirectProduct,
    Union,
    Intersection,
    // TODO Add more
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOps {
    Negation,
    BoolNot
    // TODO Add more
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conditionals {
    Equals,
    Approx,
    LessEq,
    GreaterEq,
    Less,
    Greater,
    // Maybe both depending on context?
    Congruent,
    In,
    NotIn,
    // TODO Add more
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetOps {
    Union,
    Intersection,
    SetDifference,
    // Complement is unary
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statements {
    Equivalent,
    Similar,
    SimilarEq,
    Subset,
    Superset,
    SubsetEq,
    SupersetEq,
    Parallel,
    Perpendicular,
    Models,
    Congruent,
    In,
    NotIn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Loops {
    Forall,
    Sum,
    Product,
    Union,
    Intersection,
    // Maybe
    And,
    Or,
}

// Where put Integrals and Derivative?
