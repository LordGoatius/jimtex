use std::fmt::Display;

#[derive(Debug)]
pub struct RuntimeError {
    error: RuntimeErrorTypes,
    line:  usize
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on line: {}", self.error, self.line)
    }
}

#[derive(Debug)]
pub enum RuntimeErrorTypes {
    TypeError,
    UseBeforeDefinition,
    FunctionDefinedWithNoDeclaration,
}

impl Display for RuntimeErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorTypes::TypeError => write!(f, "Type Error"),
            RuntimeErrorTypes::UseBeforeDefinition => write!(f, "Use Before Definition"),
            RuntimeErrorTypes::FunctionDefinedWithNoDeclaration => write!(f, "Function Defined with no Declaration")
        }
    }
}
