use std::collections::HashMap;
use std::fmt::Display;

use num::BigInt;

use crate::ast_types::*;
use crate::errors::*;
use crate::ast::{UnOps, BinOps};

type ExecutionResult           = Result<(), RuntimeError>;
type ExecutionResultPrint      = Result<String, RuntimeError>;
type ExecutionResultValue      = Result<Value, RuntimeError>;
type ExecutionResultNumber     = Result<Number, RuntimeError>;
type ExecutionResultExpression = Result<Expression, RuntimeError>;

#[derive(Debug, Default)]
pub struct ProgramInterpreter {
    function_definitions:  HashMap<Identifier, FunctionDefinition>,
    function_declarations: HashMap<Identifier, FunctionDeclaration>,
    variables: HashMap<Identifier, Number>,
    line: usize,
}

impl ProgramInterpreter {
    pub fn interpret_statement(&mut self, statement: Statement) -> ExecutionResult {
        match statement {
            Statement::FunctionDefinition(function_definition) => self.interpret_function_definition(function_definition),
            Statement::Declaration(declaration)                => self.interpret_declaration(declaration),
            Statement::Expression(expression)                  => {
                match self.interpret_expression(expression) {
                    Ok(string) => { println!("{string}"); Ok(()) },
                    Err(error) => Err(error),
                }
            }
        }
    }

    // Runtime determines how to handle errors

    pub fn interpret_program(&mut self, program: Program) -> ExecutionResult {
        for statement in program {
            self.line += 1;
            self.interpret_statement(statement)?;
        }
        Ok(())
    }

    fn interpret_function_definition(&mut self, definition: FunctionDefinition) -> ExecutionResult {
        // NOTE: All identifiers except ones passed as an argument must become values
        // NOTE(args): Domain is only one value, so it will be assumed all inputs are of this one
        // type (We'll say for... uh... type safety)
        let mut ignore = definition.arguments.clone();
        ignore.push(definition.identifier.clone());
        let expression = self.condense_expression(&ignore, definition.expression)?;
        self.function_definitions.insert(definition.identifier.clone(), FunctionDefinition { identifier: definition.identifier, arguments: definition.arguments, expression });
        Ok(())
    }

    fn condense_value(&self, ignore: &Vec<Identifier>, value: Value) -> ExecutionResultValue {
        match value {
            Value::Number(num)       => Ok(Value::Number(num)),
            Value::Expression(exp)   => Ok(Value::Expression(Box::new(self.condense_expression(ignore, *exp)?))),
            Value::Identifier(ident) => {
                if ignore.contains(&ident.clone()) {
                    Ok(Value::Identifier(ident))
                } else {
                    Ok(Value::Number(self.get_ident_val(ident)?))
                }
            }
        }
    }

    fn condense_expression(&self, ignore: &Vec<Identifier>, expression: Expression) -> ExecutionResultExpression {
        match expression {
            Expression::BinaryOperation(binop) => {
                let value_1 = self.condense_value(ignore, binop.value_1)?;
                let value_2 = self.condense_value(ignore, binop.value_2)?;
                Ok(Expression::BinaryOperation(BinaryOperation { value_1, binop: binop.binop, value_2 }))
            },
            Expression::UnaryOperation(unop) => {
                let value = self.condense_value(ignore, unop.value)?;
                Ok(Expression::UnaryOperation(UnaryOperation { unop: unop.unop, value }))
            },
            Expression::FunctionCall(fn_call) => {
                if ignore.contains(&fn_call.function.clone()) {
                    Ok(Expression::FunctionCall(fn_call))
                } else {
                    Ok(Expression::Value(Box::new(Value::Number(self.interpret_function_call(fn_call)?))))
                }
            },
            Expression::Value(value) => {
                match *value {
                    Value::Number(num) => Ok(Expression::Value(Box::new(Value::Number(num)))),
                    Value::Expression(exp) => Ok(self.condense_expression(ignore, *exp)?),
                    Value::Identifier(ident) => {
                        if ignore.contains(&ident.clone()) {
                            Ok(Expression::Value(Box::new(Value::Identifier(ident))))
                        } else {
                            Ok(Expression::Value(Box::new(Value::Number(self.get_ident_val(ident)?))))
                        }
                    }
                }
            },
            Expression::Conditional(conditional) => {
                Ok(Expression::Conditional(Conditional { 
                    condition:  conditional.condition, 
                    eval_true:  Box::new(self.condense_expression(ignore, *conditional.eval_true)?),
                    eval_false: Box::new(self.condense_expression(ignore, *conditional.eval_false)?),
                }))
            }
        }
    }

    fn interpret_function_call(&self, function_call: FunctionCall) -> ExecutionResultNumber {
        let mut function_scope = ProgramInterpreter::default();
        let function_defin = self.function_definitions.get(&function_call.function.clone()).ok_or(RuntimeError::new(self.line, RuntimeErrorTypes::MissingFunction(function_call.function.clone())))?;
        for (ident, value) in function_defin.arguments.clone().into_iter().zip(function_call.args.clone().into_iter()) {
            match &value {
                Value::Identifier(ident) => {
                    if self.function_declarations.contains_key(&ident) {
                        function_scope.function_declarations.insert(ident.clone(), self.function_declarations.get(&ident.clone()).unwrap().clone());
                        function_scope.function_definitions.insert(ident.clone(), self.function_definitions.get(&ident.clone()).unwrap().clone());
                    } else {
                        function_scope.variables.insert(ident.clone(), self.evaluate_value(value)?);
                    }
                },
                _ => { function_scope.variables.insert(ident, self.evaluate_value(value)?); }
            }
        }
        println!("{:#?}", self.function_declarations);
        println!("{:#?}", self.function_definitions);
        function_scope.function_declarations.insert(function_call.function.clone(), self.function_declarations.get(&function_call.function.clone()).unwrap().clone());
        function_scope.function_definitions.insert(function_call.function.clone(), self.function_definitions.get(&function_call.function.clone()).unwrap().clone());
        println!("{function_scope:?}");

        Ok(function_scope.evaluate_expression(function_defin.expression.clone())?)
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
        self.variables.insert(value_declaration.identifier.clone(), self.evaluate_value(value_declaration.value)?);
        Ok(())
    }

    fn interpret_expression(&self, expression: Expression) -> ExecutionResultPrint {
        let num = self.evaluate_expression(expression)?;
        Ok(format!("{num}"))
    }

    fn evaluate_value(&self, value: Value) -> ExecutionResultNumber {
        match value {
            Value::Number(num)       => Ok(num),
            Value::Identifier(ident) => Ok(self.get_ident_val(ident)?),
            Value::Expression(exprs) => Ok(self.evaluate_expression(*exprs)?),
        }
    }

    fn evaluate_expression(&self, expression: Expression) -> ExecutionResultNumber {
        match expression {
            Expression::Value(value) => {
                match *value {
                    Value::Number(number)    => Ok(number),
                    Value::Expression(exp)   => Ok(self.evaluate_expression(*exp)?),
                    Value::Identifier(ident) => self.get_ident_val(ident),       
                }
            },
            Expression::FunctionCall(function_call) => self.interpret_function_call(function_call),
            Expression::UnaryOperation(unop)        => self.eval_unop(unop),
            Expression::BinaryOperation(binop)      => self.eval_binop(binop),
            Expression::Conditional(conditional)    => {
                if let Number::Integer(num) = self.evaluate_value(conditional.condition)? {
                    if num == BigInt::from(0u8) {
                        Ok(self.evaluate_expression(*conditional.eval_false)?)
                    } else {
                        Ok(self.evaluate_expression(*conditional.eval_true)?)
                    }
                } else {
                    Err(RuntimeError::new(self.line, RuntimeErrorTypes::ConditionalsMustEvaluateToNumber))
                }
            }
        }
    }

    fn eval_binop(&self, binop: BinaryOperation) -> ExecutionResultNumber {
        match binop.binop {
            BinOps::Multiply => {
                let num_1 = self.evaluate_value(binop.value_1)?;
                let num_2 = self.evaluate_value(binop.value_2)?;
                match num_1 {
                    Number::Integer(num_1) => {
                        match num_2 {
                            Number::Integer(num_2) => {
                                Ok(Number::Integer(num_1 * num_2))
                            },
                            Number::Real(num_2) => {
                                let num_1 = num_1.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 * num_2))
                            },
                            _ => todo!()
                        }
                    },
                    Number::Real(num_1) => {
                        match num_2 {
                            Number::Real(num_2) => {
                                Ok(Number::Real(num_1 * num_2))
                            },
                            Number::Integer(num_2) => {
                                let num_2 = num_2.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 * num_2))
                            },
                            _ => todo!()
                        }
                    },
                    _ => todo!(),
                }
            },
            BinOps::Divide => {
                let num_1 = self.evaluate_value(binop.value_1)?;
                let num_2 = self.evaluate_value(binop.value_2)?;
                match num_1 {
                    Number::Integer(num_1) => {
                        match num_2 {
                            Number::Integer(num_2) => {
                                Ok(Number::Integer(num_1 / num_2))
                            },
                            Number::Real(num_2) => {
                                let num_1 = num_1.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 / num_2))
                            },
                            _ => todo!()
                        }
                    },
                    Number::Real(num_1) => {
                        match num_2 {
                            Number::Real(num_2) => {
                                Ok(Number::Real(num_1 / num_2))
                            },
                            Number::Integer(num_2) => {
                                let num_2 = num_2.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 / num_2))
                            },
                            _ => todo!()
                        }
                    },
                    _ => todo!(),
                }
            },
            BinOps::Addition => {
                let num_1 = self.evaluate_value(binop.value_1)?;
                let num_2 = self.evaluate_value(binop.value_2)?;
                match num_1 {
                    Number::Integer(num_1) => {
                        match num_2 {
                            Number::Integer(num_2) => {
                                Ok(Number::Integer(num_1 + num_2))
                            },
                            Number::Real(num_2) => {
                                let num_1 = num_1.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 + num_2))
                            },
                            _ => todo!()
                        }
                    },
                    Number::Real(num_1) => {
                        match num_2 {
                            Number::Real(num_2) => {
                                Ok(Number::Real(num_1 + num_2))
                            },
                            Number::Integer(num_2) => {
                                let num_2 = num_2.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 + num_2))
                            },
                            _ => todo!()
                        }
                    },
                    _ => todo!(),
                }
            },
            BinOps::Subtraction => {
                let num_1 = self.evaluate_value(binop.value_1)?;
                let num_2 = self.evaluate_value(binop.value_2)?;
                match num_1 {
                    Number::Integer(num_1) => {
                        match num_2 {
                            Number::Integer(num_2) => {
                                Ok(Number::Integer(num_1 - num_2))
                            },
                            Number::Real(num_2) => {
                                let num_1 = num_1.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 - num_2))
                            },
                            _ => todo!()
                        }
                    },
                    Number::Real(num_1) => {
                        match num_2 {
                            Number::Real(num_2) => {
                                Ok(Number::Real(num_1 - num_2))
                            },
                            Number::Integer(num_2) => {
                                let num_2 = num_2.to_string().parse::<f64>().map_err(|_| RuntimeError::new(self.line, RuntimeErrorTypes::TypeError))?; 
                                Ok(Number::Real(num_1 - num_2))
                            },
                            _ => todo!()
                        }
                    },
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
    }

    fn eval_unop(&self, unop: UnaryOperation) -> ExecutionResultNumber {
        match unop.unop {
            UnOps::Negation => {
                match unop.value {
                    Value::Number(num) => {
                        match num {
                            Number::Real(real)   => Ok(Number::Real(-real)),
                            Number::Integer(int) => Ok(Number::Integer(-int)),
                            Number::Complex(_)   => todo!(),
                            Number::Rational(_)  => todo!(),
                        }
                    },
                    Value::Identifier(ident) => self.get_ident_val(ident),
                    Value::Expression(expr)  => self.evaluate_expression(*expr),
                }
            },
            UnOps::BoolNot => todo!(),
        }
    }

    fn get_ident_val(&self, ident: Identifier) -> ExecutionResultNumber {
        if self.variables.contains_key(&ident.clone()) { Ok(self.variables[&ident.clone()].clone()) }
        else { Err(RuntimeError::new(self.line, RuntimeErrorTypes::MissingVariable(ident))) }
    }
}

impl Display for ProgramInterpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Functions: {:?}\nVariables: {:?}\n", self.function_declarations, self.variables)
    }
}
