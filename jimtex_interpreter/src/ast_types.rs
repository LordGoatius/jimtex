use num::{BigInt, BigRational};
use crate::lexer::Token;
use crate::ast::{BinOps, Conditionals, GreekLetters, SetOps, UnOps};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub program: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Declaration(Declaration),
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    Expression(Box<Expression>),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Value(Box<Value>),
    FunctionCall(FunctionCall),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub unop: UnOps,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub value_1: Value,
    pub binop:   BinOps,
    pub value_2: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub function: Identifier,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    GreekLetter(GreekLetters),
    TextIdent(String),
    SubScriptIdent(Box<SubScriptIdent>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubScriptIdent {
    pub first_ident: Identifier,
    pub secnd_ident: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(BigInt),
    Real(f64),
    Complex(Complex),
    Rational(BigRational),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    pub int_part: BigInt,
    pub decimal_part: BigInt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub value_1: Value,
    pub condition: Conditionals,
    pub value_2: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    ValueDeclaration(ValueDeclaration),
    SetDeclaration(SetDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub identifier: Identifier,
    pub domain: Token,
    pub codomain: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueDeclaration {
    pub identifier: Identifier,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetDeclaration {
    // I'll do set comprehension later, that's hard to do :(
    // I'd wanto do that functional style, and I'd need an iterator system
    ListDeclaration(ListDeclaration),
    SetOperations(SetOperations),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListDeclaration {
    pub identifier: Identifier,
    pub values: Vec<Value>
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetOperations {
    pub operation: SetOps,
    pub sets: Vec<Identifier>
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub identifier: Identifier,
    pub arguments:  Vec<Identifier>,
    pub expression: Expression
}
