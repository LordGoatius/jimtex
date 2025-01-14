use std::collections::HashMap;

use inkwell::{
    self,
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    values::{
        BasicMetadataValueEnum, BasicValueEnum, FloatValue, FunctionValue,
        PointerValue,
    },
    AddressSpace,
};

use crate::{
    ast::{BinOps, UnOps},
    ast_types::{
        BinaryOperation, Conditional, Declaration, Expression, FunctionCall, Number, Program,
        Statement, UnaryOperation, Value,
    },
};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    fn_val_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn get_function(&self, name: String) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(&name)
    }

    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_val_opt.unwrap()
    }

    fn create_entry_block_alloca(&self, name: String) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        if self.functions.contains_key(&name) {
            builder
                .build_alloca(self.context.ptr_type(AddressSpace::default()), &name)
                .unwrap()
        } else {
            builder
                .build_alloca(self.context.f64_type(), &name)
                .unwrap()
        }
    }

    pub fn build_load(&self, ptr: PointerValue<'ctx>, name: String) -> BasicValueEnum<'ctx> {
        //if self.functions.contains_key(&name) {
        //    self.builder.build_load(self.context.ptr_type(AddressSpace::default()), ptr, &name).unwrap()
        //} else {
        //    self.builder.build_load(self.context.f64_type(), ptr, &name).unwrap()
        //}
        self.builder
            .build_load(self.context.f64_type(), ptr, &name)
            .unwrap()
    }

    fn compile_value(&self, val: &Value) -> FloatValue<'ctx> {
        match val {
            Value::Identifier(ident) => {
                let name = ident.to_string();
                let var = self.variables.get(&name).unwrap();
                self.build_load(*var, name).into_float_value()
            }
            Value::Number(num) => match num {
                Number::Real(num) => self.context.f64_type().const_float(*num as f64),
                Number::Integer(int) => self
                    .context
                    .f64_type()
                    .const_float(int.to_string().parse::<f64>().unwrap()),
                _ => todo!(),
            },
            Value::Expression(expr) => self.compile_expr(expr),
        }
    }

    fn compile_expr(&self, expr: &Expression) -> FloatValue<'ctx> {
        match expr {
            Expression::Value(val) => self.compile_value(val),
            Expression::UnaryOperation(UnaryOperation { unop, value }) => {
                let val = self.compile_value(value);
                match unop {
                    UnOps::BoolNot => {
                        let cond = self
                            .builder
                            // returns true if the conditon is not zero
                            .build_float_compare(
                                inkwell::FloatPredicate::ONE,
                                val,
                                self.context.f64_type().const_zero(),
                                "ifcond",
                            )
                            .unwrap();

                        let parent = self.fn_value();
                        let then_bb = self.context.append_basic_block(parent, "then");
                        let else_bb = self.context.append_basic_block(parent, "else");
                        let cont_bb = self.context.append_basic_block(parent, "ifcont");

                        self.builder
                            .build_conditional_branch(cond, then_bb, else_bb)
                            .unwrap();

                        self.builder.position_at_end(then_bb);
                        let then_val = self.context.f64_type().const_float(0.);
                        self.builder.build_unconditional_branch(cont_bb).unwrap();

                        let then_bb = self.builder.get_insert_block().unwrap();

                        self.builder.position_at_end(else_bb);
                        let else_val = self.context.f64_type().const_float(1.);
                        self.builder.build_unconditional_branch(cont_bb).unwrap();

                        let else_bb = self.builder.get_insert_block().unwrap();

                        self.builder.position_at_end(cont_bb);

                        let phi = self
                            .builder
                            .build_phi(self.context.f64_type(), "iftmp")
                            .unwrap();

                        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);

                        phi.as_basic_value().into_float_value()
                    }
                    UnOps::Negation => self.builder.build_float_neg(val, "tmpneg").unwrap(),
                }
            }
            Expression::BinaryOperation(BinaryOperation {
                value_1,
                value_2,
                binop,
            }) => {
                let lhs = self.compile_value(value_1);
                let rhs = self.compile_value(value_2);

                match binop {
                    BinOps::Divide => self.builder.build_float_div(lhs, rhs, "tmpadd").unwrap(),
                    BinOps::Multiply => self.builder.build_float_mul(lhs, rhs, "tmpmul").unwrap(),
                    BinOps::Addition => self.builder.build_float_add(lhs, rhs, "tmpadd").unwrap(),
                    BinOps::Subtraction => {
                        self.builder.build_float_sub(lhs, rhs, "tmpsub").unwrap()
                    }
                    _ => todo!(),
                }
            }
            Expression::Conditional(Conditional {
                condition,
                eval_true,
                eval_false,
            }) => {
                let parent = self.fn_value();
                let zero = self.context.f64_type().const_zero();
                let cond = self.compile_value(condition);
                let cond = self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, cond, zero, "ifcond")
                    .unwrap();

                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let cont_bb = self.context.append_basic_block(parent, "ifcont");

                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb)
                    .unwrap();

                self.builder.position_at_end(then_bb);
                let then_val = self.compile_expr(eval_true);
                self.builder.build_unconditional_branch(cont_bb).unwrap();

                let then_bb = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(else_bb);
                let else_val = self.compile_expr(eval_false);
                self.builder.build_unconditional_branch(cont_bb).unwrap();

                let else_bb = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(cont_bb);

                let phi = self
                    .builder
                    .build_phi(self.context.f64_type(), "iftmp")
                    .unwrap();

                phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);

                phi.as_basic_value().into_float_value()
            }
            Expression::FunctionCall(FunctionCall { function, args }) => {
                let fun = self.functions.get(&function.to_string()).unwrap();
                let args: Vec<BasicMetadataValueEnum<'ctx>> = args
                    .iter()
                    .map(|val| {
                        // This Lambda Must:
                        // - Turn function pointers into pointers
                        // - Turn double expressions into doubles
                        match val {
                            tempval @ Value::Identifier(ident) => {
                                match self.functions.get(&ident.to_string()) {
                                    Some(fn_passed) => {
                                        fn_passed.as_global_value().as_pointer_value().into()
                                    }
                                    None => self.compile_value(tempval).into(),
                                }
                            }
                            float => self.compile_value(float).into(),
                        }
                    })
                    .collect();
                self.builder
                    .build_call(*fun, args.as_slice(), "tmp")
                    .unwrap();
                todo!()
            }
        }
    }

    fn compile_program(&self, program: Program) {
        for statement in program {
            match statement {
                // compile function
                Statement::FunctionDefinition(..) => todo!(),
                // evaluate, then printan expression
                Statement::Expression(expr) => {
                    let val = self.compile_expr(&expr);
                    let per = self.context.i8_type().const_int('%'.into(), false);
                    let g = self.context.i8_type().const_int('g'.into(), false);
                    let z = self.context.i8_type().const_zero();
                    let arr = self.context.i8_type().const_array(&[per, g, z]);
                    // may need this:
                    let ptr = self.builder.build_alloca(arr.get_type(), "tmpptr").unwrap();
                    self.builder.build_store(ptr, arr).unwrap();
                    self.builder.build_call(
                        self.functions["printf"],
                        &[ptr.into(), val.into()],
                        "tmpcall",
                    ).unwrap();
                }
                // declare a variable
                Statement::Declaration(decl) => match decl {
                    Declaration::ValueDeclaration(..) => todo!(),
                    Declaration::FunctionDeclaration(..) => todo!(),
                    Declaration::SetDeclaration(..) => todo!(),
                },
            }
        }
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        program: Program,
    ) {
        let mut compiler = Compiler {
            context,
            builder,
            module,

            fn_val_opt: None,
            functions: HashMap::new(),
            variables: HashMap::new(),
        };

        let print = module.add_function(
            "printf",
            context
                .i64_type()
                .fn_type(&[context.ptr_type(AddressSpace::default()).into()], true),
            Some(Linkage::External),
        );

        compiler.functions.insert("printf".to_string(), print);

        //let main_type = context.i64_type().fn_type(&[], false);
        //let main_fn = module.add_function("main", main_type, None);
        //compiler.fn_val_opt = Some(main_fn);

        //let entry = context.append_basic_block(main_fn, "entry");

        // NOTE: Compile everything inside this function
        compiler.compile_program(program);

        // FIXME: Never gonna work yet, functions are nested
        // idk how to fix that lmao I'll figure it out
        // maybe I just don't use the main fn lol
        //builder.position_at_end(entry);
        let ret = builder.build_alloca(context.i64_type(), "return").unwrap();
        builder.build_store(ret, context.i64_type().const_zero());
        builder.build_return(Some(&ret)).unwrap();

        todo!()
    }
}
