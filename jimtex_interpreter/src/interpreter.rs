use std::collections::HashMap;
use std::fmt::Display;

use crate::ast_types::*;
use crate::errors::*;

type ExecutionResult = Result<(), RuntimeError>;

#[derive(Debug)]
struct ProgramInterpreter {
    function_definitions:  HashMap<Identifier, FunctionDefinition>,
    function_declarations: HashMap<Identifier, FunctionDeclaration>,
    variables: HashMap<Identifier, Value>
}

impl ProgramInterpreter {
    pub fn interpret_statement(&mut self, statement: Statement) -> ExecutionResult {
        match statement {
            Statement::FunctionDefinition(function_definition) => self.interpret_function_definition(function_definition),
            Statement::Declaration(declaration)                => self.interpret_declaration(declaration),
            Statement::Expression(expression)                  => self.iterpret_expression(expression),
        }
    }

    pub fn interpret_program(&mut self, program: Program) -> ExecutionResult {
        for statement in program {
            self.interpret_statement(statement)?;
        }
        Ok(())
    }

    fn interpret_function_definition(&mut self, definition: FunctionDefinition) -> ExecutionResult {
        todo!()
    }

    fn interpret_declaration(&mut self, declaration: Declaration) -> ExecutionResult {
        match declaration {
            Declaration::SetDeclaration(_) => todo!(),
            Declaration::ValueDeclaration(value_declaration)       => self.interpret_value_declaration(value_declaration),
            Declaration::FunctionDeclaration(function_declaration) => self.interpret_funct_declaration(function_declaration),
        }
    }

    fn interpret_funct_declaration(&mut self, function_declaration: FunctionDeclaration) -> ExecutionResult {
        self.function_declarations.insert(function_declaration.identifier.clone(), function_declaration);
        Ok(())
    }

    fn interpret_value_declaration(&mut self, value_declaration: ValueDeclaration) -> ExecutionResult {
        // NOTE: Declared values MUST evaluate to a number/specific value at runtime
        todo!()
    }

    fn iterpret_expression(&self, expression: Expression) -> ExecutionResult {
        todo!()
    }

    fn interpret_function_call(&self, function_call: FunctionCall) -> ExecutionResult {
        todo!()
    }
}

impl Display for ProgramInterpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Functions: {:?}\nVariables: {:?}\n", self.function_declarations, self.variables)
    }
}
