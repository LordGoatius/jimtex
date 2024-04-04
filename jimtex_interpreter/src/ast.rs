#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum BinOps {
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

#[derive(Debug, Clone)]
pub enum UnOps {
    Negation,
    BoolNot
    // TODO Add more
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
