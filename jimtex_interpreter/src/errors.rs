use std::fmt::Display;

use crate::ast_types::Identifier;

#[derive(Debug)]
pub struct RuntimeError {
    error: RuntimeErrorTypes,
    line:  usize
}

impl RuntimeError {
    pub fn new(line: usize, error: RuntimeErrorTypes) -> Self {
        Self { error, line }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on line: {}", self.error, self.line)
    }
}

#[derive(Debug)]
pub enum RuntimeErrorTypes {
    TypeError,
    MissingVariable(Identifier),
    MissingFunction(Identifier),
    UseBeforeDefinition,
    FunctionDefinedWithNoDeclaration,
    ConditionalsMustEvaluateToNumber,
}

impl Display for RuntimeErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorTypes::TypeError                        => write!(f, "Type Error"),
            RuntimeErrorTypes::UseBeforeDefinition              => write!(f, "Use Before Definition"),
            RuntimeErrorTypes::FunctionDefinedWithNoDeclaration => write!(f, "Function Defined with no Declaration"),
            RuntimeErrorTypes::MissingVariable(ident)           => write!(f, "Variable {ident:?} does not exist"),
            RuntimeErrorTypes::MissingFunction(ident)           => write!(f, "Function {ident:?} does not exist"),
            RuntimeErrorTypes::ConditionalsMustEvaluateToNumber => write!(f, "Conditionals must evaluate to a number"),
        }
    }
}
