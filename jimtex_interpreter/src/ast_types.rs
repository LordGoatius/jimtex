use num::{BigInt, BigRational};

use crate::ast::{BinOps, Conditionals, GreekLetters, SetOps, UnOps};

pub struct Program {
    program: Vec<Statement>,
}

pub enum Statement {
    Value(Value),
    Expression(Expression),
    Conditional(Condition),
    Declaration(Declaration),
    FunctionDefinition(FunctionDefinition),
}

pub enum Value {
    Number(Number),
    Expression(Box<Expression>),
    Identifier(Identifier),
}

pub enum Expression {
    FunctionCall(FunctionCall),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}

pub struct UnaryOperation {
    unop: UnOps,
    value: Value,
}

pub struct BinaryOperation {
    value_1: Value,
    binop:   BinOps,
    value_2: Value,
}

pub struct FunctionCall {
    function: Identifier,
    args: Vec<Value>,
}

pub enum Identifier {
    GreekLetter(GreekLetters),
    TextIdent(String),
    SubScriptIdent(Box<SubScriptIdent>),
}

pub struct SubScriptIdent {
    first_ident: Identifier,
    secnd_ident: Identifier,
}

pub enum Number {
    Integer(BigInt),
    Real(f64),
    Complex(Complex),
    Rational(BigRational),
}

pub struct Complex {
    real: f64,
    imag: f64,
}

pub struct Condition {
    value_1: Value,
    condition: Conditionals,
    value_2: Value,
}

pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    ValueDeclaration(ValueDeclaration),
    SetDeclaration(SetDeclaration),
}

pub struct FunctionDeclaration {
    identifier: Identifier,
    domain: Number,
    codomain: Number,
}

pub struct ValueDeclaration {
    identifier: Identifier,
    value: Value,
}

pub enum SetDeclaration {
    // I'll do set comprehension later, that's hard to do :(
    // I'd wanto do that functional style, and I'd need an iterator system
    ListDeclaration(ListDeclaration),
    SetOperations(SetOperations),
}

pub struct ListDeclaration {
    identifier: Identifier,
    values: Vec<Value>
}

pub struct SetOperations {
    operation: SetOps,
    sets: Vec<Identifier>
}

pub struct FunctionDefinition {
    identifier: Identifier,
    expression: Expression
}
