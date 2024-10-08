use crate::lexer::{Operator, Token, TokenString};
use crate::ast::*;
use crate::ast_types::*;

pub fn parse_to_ast(tokens: TokenString) -> Program {
    let mut program = vec![];
    for slice in into_slices(tokens) {
        match slice_type(&slice) {
           SliceType::Declaration        => program.push(Statement::Declaration(parse_declaration(slice))),
           SliceType::FunctionDefinition => program.push(Statement::FunctionDefinition(parse_function_def(slice))),
           SliceType::Expression         => program.push(Statement::Expression(parse_expression(slice))),
        }
    }

    Program { program }
}

fn slice_type(tokens: &TokenString) -> SliceType {
    if let Some((left, _)) = tokens.split_once(|token| *token == Token::Equals) {
        if left.contains(&Token::LeftParen) {
            SliceType::FunctionDefinition
        } else {
            SliceType::Declaration
        }
    } else if tokens.contains(&Token::Colon) {
        SliceType::Declaration 
    } else {
        SliceType::Expression
    }
}

enum SliceType {
    Declaration,
    FunctionDefinition,
    Expression,
}

fn parse_expression(tokens: TokenString) -> Expression {
    parse_conditionals(tokens)
}

fn parse_conditionals(tokens: TokenString) -> Expression {
    if let Some(Token::If) = tokens.first() {
        if let [condition, true_exp, false_exp] = tokens
            .clone()
            .into_iter()
            .filter(|token| *token != Token::If)
            .collect::<Vec<Token>>()
            .split(|token| *token == Token::Then || *token == Token::Else)
            .collect::<Vec<_>>()[..] {
            Expression::Conditional(Conditional { condition: parse_value(condition.to_vec()), eval_true: Box::new(parse_expression(true_exp.to_vec())), eval_false: Box::new(parse_expression(false_exp.to_vec())) })
        } else {
            panic!("should split into three")
        }
    } else {
        Expression::Value(Box::new(parse_value(tokens)))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Three,
    Two,
    One,
    None
}

fn precedence(token: Option<&Token>) -> Precedence {
    if token.is_none() {
        return Precedence::None;
    }
    match token.unwrap() {
        Token::Operator(op) => {
            match op {
                Operator::Mul |
                Operator::Div => Precedence::Two,
                Operator::Add | 
                Operator::Sub => Precedence::Three,
            }
        },
        Token::Exponent => Precedence::Three,
        Token::BinOp(binop) => {
            match binop {
                BinOps::Multiply |
                BinOps::Divide |
                BinOps::BoolAnd |
                BinOps::BoolXor |
                BinOps::ExternalDirectProduct |
                BinOps::InternalDirectProduct |
                BinOps::Subtraction | 
                BinOps::Addition => Precedence::Two,
                BinOps::Union  |
                BinOps::Intersection |
                BinOps::BoolOr |
                BinOps::PlusMinus |
                BinOps::SetDifference => Precedence::One,
            }
        },
        Token::UnOps(_) => Precedence::Three,
        _ => Precedence::None,
    }
}

fn make_function_calls(tokens: TokenString) -> TokenString {
    let mut res: Vec<Token> = vec![];
    let copy = tokens.clone();
    let mut ignore = 0;

    for (i, token) in tokens.into_iter().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }
        match token {
            Token::GreekLetter(_)
            | Token::Text(_) => {
                if let Some(Token::LeftParen) = copy.get(i+1) {
                    let mut balanced = 1;
                    let mut count = 1;

                    while balanced > 0 {
                        if let Some(Token::RightParen) = copy.get(i+1+count) {
                            balanced -= 1;
                        }
                        count += 1;
                    }

                    res.push(Token::FunctionCall(FunctionCall { 
                        function: parse_identifier(vec![token]), 
                        args: copy.clone()[(i+2)..=(i+count-1)]
                            .split(|token| *token == Token::Comma)
                            .map(|slice| parse_value(slice.to_vec()))
                            .collect()
                    }));
                    ignore = count;
                } else {
                    res.push(copy[i].clone());
                }
            }
            token => res.push(token),
        }
    }

    res
}

fn parse_value(tokens: TokenString) -> Value {
    if let Some(Token::If) = tokens.first() {
        if let [condition, true_exp, false_exp] = tokens
            .clone()
            .into_iter()
            .filter(|token| *token != Token::If)
            .collect::<Vec<Token>>()
            .split(|token| *token == Token::Then || *token == Token::Else)
            .collect::<Vec<_>>()[..] {
            Value::Expression(Box::new(Expression::Conditional(Conditional { condition: parse_value(condition.to_vec()), eval_true: Box::new(parse_expression(true_exp.to_vec())), eval_false: Box::new(parse_expression(false_exp.to_vec())) })))
        } else {
            panic!("should split into three")
        }
    } else {
        let mut res: TokenString = vec![];
        let tokens = make_real_numbers(tokens);
        let tokens = make_function_calls(tokens);
        // Base cases of number and identifier
        let mut stack: TokenString = vec![];

        for token in tokens {
            match token {
                Token::Real(number) => {
                    res.push(Token::Real(number));
                },
                Token::Number(number) => {
                    res.push(Token::Number(number.parse().unwrap()));
                },
                Token::Text(text) => {
                    res.push(Token::Text(text));
                },
                Token::GreekLetter(letter) => {
                    res.push(Token::GreekLetter(letter));
                },
                Token::FunctionCall(call) => {
                    res.push(Token::FunctionCall(call));
                },
                //assume identifier
                Token::Operator(_) => {
                    while !stack.is_empty() && precedence(Some(&token)) <  precedence(stack.last())
                                            || precedence(Some(&token)) == precedence(stack.last()) 
                                            && associativity(&token) == Associativity::Left {
                        res.push(stack.pop().unwrap())
                    }
                    stack.push(token);
                },
                Token::BinOp(_) => {
                    while !stack.is_empty() && precedence(Some(&token)) <  precedence(stack.last())
                                            || precedence(Some(&token)) == precedence(stack.last()) 
                                            && associativity(&token) == Associativity::Left {
                        res.push(stack.pop().unwrap())
                    }
                    stack.push(token);
                },
                Token::UnOps(_) => {
                    while !stack.is_empty() && precedence(Some(&token)) <  precedence(stack.last())
                                            || precedence(Some(&token)) == precedence(stack.last()) 
                                            && associativity(&token) == Associativity::Left {
                        res.push(stack.pop().unwrap())
                    }
                    stack.push(token);
                },
                Token::LeftParen => {
                    stack.push(Token::LeftParen);
                },
                Token::RightParen => {
                    while !stack.is_empty() && stack.last() != Some(&Token::LeftParen) {
                        res.push(stack.pop().unwrap());
                    }
                }
                _ => panic!(),
                // assume no Subscript identifiers, I just want a working parser for some things
            }
        }

        while let Some(element) = stack.pop() {
            res.push(element);
        }

        // while !stack.is_empty() {
        //     res.push(stack.pop().unwrap());
        // }

        // Now in reverse polish notation
        
        let res = res.into_iter().filter(|token| *token != Token::LeftParen).collect::<TokenString>();

        let mut res_2: Vec<Value> = vec![];

        for token in res {
            match token {
                Token::Real(number) => {
                    res_2.push(Value::Number(Number::Real(number)));
                },
                Token::Number(number) => {
                    res_2.push(Value::Number(Number::Integer(number.parse().unwrap())));
                },
                Token::Text(text) => {
                    res_2.push(Value::Identifier(Identifier::TextIdent(text)));
                },
                Token::GreekLetter(letter) => {
                    res_2.push(Value::Identifier(Identifier::GreekLetter(letter)));
                },
                Token::FunctionCall(call) => {
                    res_2.push(Value::Expression(Box::new(Expression::FunctionCall(call))));
                },
                //assume identifier
                Token::Operator(operator) => {
                    let value_2 = res_2.pop().unwrap();
                    let value_1 = res_2.pop().unwrap();
                    res_2.push(Value::Expression(Box::new(Expression::BinaryOperation(BinaryOperation { 
                        value_1,
                        binop: token_op_to_binop(operator), 
                        value_2,
                    }))))
                },
                Token::BinOp(binop) => {
                    let value_2 = res_2.pop().unwrap();
                    let value_1 = res_2.pop().unwrap();
                    res_2.push(Value::Expression(Box::new(Expression::BinaryOperation(BinaryOperation { 
                        value_1,
                        binop, 
                        value_2
                    }))))
                },
                Token::UnOps(unop) => {
                    res_2.push(Value::Expression(Box::new(Expression::UnaryOperation(UnaryOperation { 
                        value: real_num_text_greek_to_val(stack.pop().unwrap()),
                        unop,
                    }))))
                },
                _ => (),
                // assume no Subscript identifiers, I just want a working parser for some things
            }
        }
        res_2.last().unwrap().clone()
    }
}

fn real_num_text_greek_to_val(token: Token) -> Value {
    match token {
        Token::Real(number) => Value::Number(Number::Real(number)),
        Token::Number(number) => Value::Number(Number::Integer(number.parse().unwrap())),
        Token::Text(text) => Value::Identifier(Identifier::TextIdent(text)),
        Token::GreekLetter(letter) => Value::Identifier(Identifier::GreekLetter(letter)),
        _ => panic!()
    }
}

fn token_op_to_binop(token: Operator) -> BinOps {
    match token {
        Operator::Mul => BinOps::Multiply,
        Operator::Add => BinOps::Addition,
        Operator::Div => BinOps::Divide,
        Operator::Sub => BinOps::Subtraction,
    }
}

#[derive(PartialEq, Eq)]
enum Associativity {
    Left,
    Right
}

fn associativity(token: &Token) -> Associativity {
    if let Token::Exponent = token {
        return Associativity::Right;
    }
    Associativity::Left
}

fn text_or_greek_to_ident(token: Token) -> Identifier {
    match token {
        Token::Text(text) => Identifier::TextIdent(text),
        Token::GreekLetter(letter) => Identifier::GreekLetter(letter),
        _ => unreachable!()
    }
}

fn make_real_numbers(tokens: TokenString) -> TokenString {
    let mut ret = vec![];
    let string = tokens.clone();

    let mut ignore = 0;
    
    for (i, token) in tokens.into_iter().enumerate() {
        if ignore > 0 {
            ignore -= 1;
            continue;
        }

        match token {
            Token::Number(integer_part) => {
                if let Some(Token::Period) = string.get(i+1) {
                    if let Some(Token::Number(real_part)) = string.get(i+2) {
                        ret.push(Token::Real(format!("{integer_part}.{real_part}").parse::<f64>().unwrap()));
                        ignore += 2;
                    }
                } else {
                    ret.push(Token::Number(integer_part));
                }
            },
            literally_anything_else => ret.push(literally_anything_else),
        }
    }

    ret
}

fn parse_function_def(tokens: TokenString) -> FunctionDefinition {
    let (signature, expression) = tokens.split_once(|token| *token == Token::Equals).unwrap();
    // f(x) = x^2
    let (name, values) = signature.split_once(|token| *token == Token::LeftParen).unwrap();

    let name = text_or_greek_to_ident(name
        .iter()
        .filter(|token| {
            if let Token::GreekLetter(_) = token {
                return true;
            }
            if let Token::Text(_) = token {
                return true;
            }
            false
        })
        .cloned()
        .collect::<Vec<Token>>()
        .first()
        .unwrap().clone());

    let args = values.iter()
        .filter(|token| {
            if let Token::GreekLetter(_) = token {
                return true;
            }
            if let Token::Text(_) = token {
                return true;
            }
            false
        })
        .map(|token| text_or_greek_to_ident(token.clone()))
        .collect::<Vec<Identifier>>();
    
    // Name must be text or greek letter

    FunctionDefinition { 
        identifier: name,
        arguments: args, 
        expression: parse_expression(expression.to_vec())
    }
}

fn parse_declaration(tokens: TokenString) -> Declaration {
    match tokens.split_once(|elem| *elem == Token::Equals) {
        Some((ident, expression)) => {
            let identifier = parse_identifier(ident.to_vec());
            // Value or set now
            if expression.contains(&Token::RightBrace) {
                todo!()
            } 
            // Value
            let value = parse_value(expression.to_vec());
            Declaration::ValueDeclaration(ValueDeclaration { identifier, value })
        }
        None => {
            // Probably a function 🤷
            let tokens = tokens.into_iter().filter(|token| *token != Token::Space).collect::<Vec<Token>>();
            // Gonna assume everything is correct syntax
            let (ident, definition) = tokens.split_once(|token| *token == Token::Colon).unwrap();
            let identifier = parse_identifier(ident.to_vec());
            let (domain, codomain) = definition.split_once(|token| *token == Token::RightArrow).unwrap();
            // Assume domain and codomain are in  Token::[RealNumbers, NatrualNumbers,
            // ComplexNumbers, RationalNumbers, Integers]
            let domain   = domain.first().unwrap().clone();
            let codomain = codomain.first().unwrap().clone();
            Declaration::FunctionDeclaration(FunctionDeclaration { identifier, domain, codomain })
        }
    }
}

fn parse_identifier(tokens: TokenString) -> Identifier {
    if tokens.contains(&Token::Subscript) {
        let (first_ident, secnd_ident) = tokens.split_once(|elem| *elem == Token::Subscript).unwrap();
        let first_ident = parse_identifier(first_ident.to_vec());
        let secnd_ident = parse_identifier(secnd_ident.to_vec());

        return Identifier::SubScriptIdent(Box::new(
            SubScriptIdent { 
                first_ident,
                secnd_ident,
            }
        ));
    }

    match tokens.first() {
        Some(Token::Text(text)) => {
            Identifier::TextIdent(text.to_owned())
        },
        Some(Token::GreekLetter(greek_letter)) => {
            Identifier::GreekLetter(greek_letter.to_owned())
        },
        _ => panic!("Invalid syntax")
    }
}

fn into_slices(tokens: TokenString) -> Vec<Vec<Token>> {
    let mut slices: Vec<Vec<Token>> = vec![];

    let mut curr_slice: Vec<Token> = vec![];

    let mut balanced = 0;

    for token in tokens {
        match token {
            Token::Comma => {
                if balanced == 0 {
                    slices.push(curr_slice);
                    curr_slice = vec![];
                } else {
                    curr_slice.push(Token::Comma);
                }
            },
            Token::LeftBrace  => balanced += 1,
            Token::RightBrace => balanced -= 1,
            Token::LeftParen  => {
                balanced += 1;
                curr_slice.push(Token::LeftParen);
            }
            Token::RightParen => {
                balanced -= 1;
                curr_slice.push(Token::RightParen);
            }
            token => curr_slice.push(token),
        }
    }

    slices.push(curr_slice);

    slices.into_iter().filter(|slice| !slice.is_empty()).collect()
}
