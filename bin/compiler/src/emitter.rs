use std::collections::HashMap;

use crate::{
    ast::{BinaryOp, Expression, FunctionParam, Statement, Ty},
    parser::Module,
};
use ir::function_builder::FunctionBuilder;

impl Ty {
    fn to_ir(&self) -> ir::ty::TypeKind {
        use crate::ast::Builtin;
        use ir::ty::TypeKind;
        match self {
            Ty::Builtin(builtin) => match builtin {
                Builtin::Void => todo!(),
                Builtin::Bool => TypeKind::Integer {
                    num_bits: 1,
                    is_signed: false,
                },
                Builtin::U8 => TypeKind::Integer {
                    num_bits: 8,
                    is_signed: false,
                },
                Builtin::U16 => TypeKind::Integer {
                    num_bits: 16,
                    is_signed: false,
                },
                Builtin::U32 => TypeKind::Integer {
                    num_bits: 32,
                    is_signed: false,
                },
                Builtin::U64 => TypeKind::Integer {
                    num_bits: 64,
                    is_signed: false,
                },
                Builtin::I8 => TypeKind::Integer {
                    num_bits: 8,
                    is_signed: true,
                },
                Builtin::I16 => TypeKind::Integer {
                    num_bits: 16,
                    is_signed: true,
                },
                Builtin::I32 => TypeKind::Integer {
                    num_bits: 32,
                    is_signed: true,
                },
                crate::ast::Builtin::I64 => TypeKind::Integer {
                    num_bits: 64,
                    is_signed: true,
                },
                crate::ast::Builtin::F32 => todo!(),
                crate::ast::Builtin::F64 => todo!(),
            },
            Ty::NamedType { name } => todo!(),
            Ty::Array { ty, size } => todo!(),
            Ty::Tuple { types } => todo!(),
            Ty::Function { ret, params } => todo!(),
            Ty::Struct { fields } => todo!(),
        }
    }
}

#[derive(Clone)]
enum CodegenValue {
    RValue { value: ir::value::Value },
    LValue { value: ir::value::Value },
}

impl CodegenValue {
    pub fn extract(&self, builder: &mut FunctionBuilder<'_>) -> ir::value::Value {
        match self {
            CodegenValue::RValue { value, .. } => *value,
            CodegenValue::LValue { value, .. } => builder.load(*value),
        }
    }

    pub fn value(&self) -> ir::value::Value {
        match self {
            CodegenValue::RValue { value, .. } => *value,
            CodegenValue::LValue { value, .. } => *value,
        }
    }
}

///
enum Symbol {
    Function {
        ir: ir::function::Function,
        ret: Ty,
        params: Vec<FunctionParam>,
    },
    Value(CodegenValue),
}

///
#[derive(Default)]
struct Types {
    type_to_ir: HashMap<Ty, ir::ty::Type>,
}

impl Types {
    ///
    pub fn get_or_create(
        &mut self,
        ty: Ty,
        create: impl FnOnce() -> ir::ty::Type,
    ) -> ir::ty::Type {
        let ty = self.type_to_ir.entry(ty).or_insert_with(|| create());

        *ty
    }

    //
    pub fn get(&self, ty: &Ty) -> Option<ir::ty::Type> {
        self.type_to_ir.get(ty).cloned()
    }
}

///
#[derive(Default)]
struct Variables {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl Variables {
    pub fn insert(&mut self, name: &str, symbol: Symbol) {
        let is_inserted = self
            .scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), symbol)
            .is_none();

        assert!(is_inserted)
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        assert!(self.scopes.pop().is_some())
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }

        None
    }
}

pub struct Emitter {
    context: ir::context::Context,
    variables: Variables,
    types: Types,
}

impl Emitter {
    pub fn new() -> Self {
        let mut ret = Self {
            context: ir::context::Context::new(),
            variables: Variables::default(),
            types: Types::default(),
        };

        ret.types
            .get_or_create(Ty::Builtin(crate::ast::Builtin::U32), || {
                ret.context
                    .create_type(Ty::Builtin(crate::ast::Builtin::U32).to_ir())
            });

        ret
    }

    ///
    pub fn emit(&mut self, module: &Module) {
        self.variables.enter_scope();
        for decl in module.declarations.iter() {
            match decl {
                Statement::Function { name, ty, body } => {
                    self.emit_function(name, ty, body)
                }
                Statement::Struct {} => todo!(),
                _ => panic!(),
            }
        }
        self.variables.exit_scope();

        self.context.validate();
        self.context.dump_ir(std::path::Path::new("test.ir"));
    }

    ///
    fn emit_function(&mut self, name: &str, ty: &Ty, body: &Statement) {
        self.variables.enter_scope();
        let function = self.emit_function_type(name, ty);
        let Ty::Function { ret, params } = ty else {
            panic!()
        };

        self.variables.insert(
            name,
            Symbol::Function {
                ir: function,
                ret: ret.as_ref().clone(),
                params: params.to_vec(),
            },
        );

        let mut builder = self.context.builder(function);
        let prologue = builder.create_label("prologue");
        builder.set_insert_point(prologue);

        for (i, param) in params.iter().enumerate() {
            let value = builder.parameter(i);

            let ty = self.types.get(param.ty.as_ref()).unwrap();
            let ptr = builder.stack_alloc(ty, 1);

            builder.store(ptr, value);

            self.variables.insert(
                &param.name,
                Symbol::Value(CodegenValue::LValue { value: ptr }),
            );
        }

        Self::emit_block(&mut self.variables, &mut self.types, &mut builder, body);

        self.variables.exit_scope();
    }

    ///
    fn emit_function_type(&mut self, name: &str, ty: &Ty) -> ir::function::Function {
        let Ty::Function { ret, params } = ty else {
            panic!()
        };
        let parameter_types = params
            .iter()
            .map(|param| {
                self.types.get_or_create(param.ty.as_ref().clone(), || {
                    self.context.create_type(param.ty.to_ir())
                })
            })
            .collect::<Vec<_>>();

        let return_type = self.types.get_or_create(ret.as_ref().clone(), || {
            self.context.create_type(ret.to_ir())
        });

        self.context
            .create_function(name, Some(return_type), &parameter_types)
    }

    ///
    fn emit_block(
        variables: &mut Variables,
        types: &mut Types,
        builder: &mut FunctionBuilder<'_>,
        body: &Statement,
    ) {
        variables.enter_scope();

        match body {
            Statement::Block { nodes } => {
                variables.enter_scope();
                for stmt in nodes.iter() {
                    Self::emit_block(variables, types, builder, stmt);
                }
                variables.exit_scope();
            }
            Statement::If {
                condition,
                on_true,
                on_false,
            } => Self::emit_if(
                variables,
                types,
                builder,
                condition,
                on_true,
                on_false.as_deref(),
            ),
            Statement::For { value, range, body } => {
                Self::emit_for(variables, types, builder, value, range, body)
            }
            Statement::While { condition, body } => todo!(),
            Statement::Return { expr } => {
                if let Some(expr) = expr {
                    let value =
                        Self::emit_expression(variables, types, builder, expr).unwrap();
                    let value = value.extract(builder);
                    builder.ret(Some(value));
                } else {
                    builder.ret(None);
                }
            }
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
            Statement::Assign { dst, src } => {
                let dst = Self::emit_expression(variables, types, builder, dst).unwrap();
                let src = Self::emit_expression(variables, types, builder, src).unwrap();

                let src_value = src.extract(builder);
                builder.store(dst.value(), src_value);
            }
            Statement::CompoundAssign { dst, op, src } => {
                let dst = Self::emit_expression(variables, types, builder, dst).unwrap();
                let src = Self::emit_expression(variables, types, builder, src).unwrap();

                let dst_value = dst.extract(builder);
                let src_value = src.extract(builder);

                use crate::ast::AssignOp;
                let value = match op {
                    AssignOp::Add => builder.add(dst_value, src_value),
                    AssignOp::Sub => builder.sub(dst_value, src_value),
                    AssignOp::Mul => builder.mul(dst_value, src_value),
                    AssignOp::Div => builder.div(dst_value, src_value),
                    AssignOp::Mod => builder.mod_(dst_value, src_value),
                    AssignOp::And => builder.and(dst_value, src_value),
                    AssignOp::Xor => builder.xor(dst_value, src_value),
                    AssignOp::Or => builder.or(dst_value, src_value),
                    AssignOp::Not => todo!(),
                };

                builder.store(dst.value(), value);
            }
            Statement::Let { name, ty, expr } => {
                let ty =
                    types.get_or_create(ty.clone(), || builder.create_type(ty.to_ir()));

                let src = Self::emit_expression(variables, types, builder, expr).unwrap();
                let dst = builder.stack_alloc(ty, 1);
                let src_value = src.extract(builder);
                builder.store(dst, src_value);

                variables
                    .insert(name, Symbol::Value(CodegenValue::LValue { value: dst }));
            }
            _ => panic!(),
        }

        variables.exit_scope();
    }

    ///
    fn emit_if(
        variables: &mut Variables,
        types: &mut Types,
        builder: &mut FunctionBuilder<'_>,
        condition: &Expression,
        on_true: &Statement,
        on_false: Option<&Statement>,
    ) {
        variables.enter_scope();

        let on_true_label = builder.create_label("on_true");
        let on_false_label = builder.create_label("on_false");
        let end_label = builder.create_label("end");

        {
            let condition =
                Self::emit_expression(variables, types, builder, condition).unwrap();
            let condition_value = condition.extract(builder);
            builder.branch_conditional(condition_value, on_true_label, on_false_label);
        }

        {
            builder.set_insert_point(on_true_label);
            Self::emit_block(variables, types, builder, on_true);
            builder.branch(end_label);
        }

        if let Some(on_false) = on_false {
            builder.set_insert_point(on_false_label);
            Self::emit_block(variables, types, builder, on_false);
            builder.branch(end_label);
        }

        builder.set_insert_point(end_label);

        variables.exit_scope();
    }

    ///
    fn emit_for(
        variables: &mut Variables,
        types: &mut Types,
        builder: &mut FunctionBuilder<'_>,
        value: &Expression,
        range: &Expression,
        body: &Statement,
    ) {
        variables.enter_scope();

        let loop_body = builder.create_label("loop_body");
        let loop_continue = builder.create_label("loop_continue");
        let loop_end = builder.create_label("loop_end");

        let (from, to) = {
            let Expression::Range { from, to } = range else {
                panic!()
            };

            let from = Self::emit_expression(variables, types, builder, from).unwrap();
            let to = Self::emit_expression(variables, types, builder, to).unwrap();

            (from, to)
        };

        let ty = types.get_or_create(Ty::Builtin(crate::ast::Builtin::U32), || {
            builder.create_type(Ty::Builtin(crate::ast::Builtin::U32).to_ir())
        });

        let value = {
            let Expression::Identifier { name } = value else {
                panic!()
            };
            let value = builder.stack_alloc(ty, 1);
            let from_value = from.extract(builder);
            builder.store(value, from_value);
            let value = CodegenValue::LValue { value };
            variables.insert(name, Symbol::Value(value.clone()));
            value
        };

        builder.branch(loop_body);
        builder.set_insert_point(loop_body);
        let condition = {
            let value = value.extract(builder);
            let to_value = to.extract(builder);
            builder.compare_lt(value, to_value)
        };
        builder.branch_conditional(condition, loop_continue, loop_end);

        {
            builder.set_insert_point(loop_continue);
            Self::emit_block(variables, types, builder, body);

            let one = builder
                .alloc_constant(ir::constant::ConstantValue::Integer { ty, value: 1 });
            let vv = value.extract(builder);
            let plus_one = builder.add(vv, one);
            builder.store(value.value(), plus_one);
            builder.branch(loop_body);
        }

        builder.set_insert_point(loop_end);

        variables.exit_scope();
    }

    ///
    fn emit_expression(
        variables: &mut Variables,
        types: &mut Types,
        builder: &mut FunctionBuilder<'_>,
        expr: &Expression,
    ) -> Option<CodegenValue> {
        match expr {
            Expression::Binary { lhs, op, rhs } => {
                let lhs = Self::emit_expression(variables, types, builder, lhs).unwrap();
                let rhs = Self::emit_expression(variables, types, builder, rhs).unwrap();

                let lhs_value = lhs.extract(builder);
                let rhs_value = rhs.extract(builder);

                let value = match op {
                    BinaryOp::Add => builder.add(lhs_value, rhs_value),
                    BinaryOp::Sub => builder.sub(lhs_value, rhs_value),
                    BinaryOp::Mul => builder.mul(lhs_value, rhs_value),
                    BinaryOp::Mod => builder.mod_(lhs_value, rhs_value),
                    BinaryOp::Div => builder.div(lhs_value, rhs_value),
                    BinaryOp::Shr => builder.shr(lhs_value, rhs_value),
                    BinaryOp::Shl => builder.shl(lhs_value, rhs_value),
                    BinaryOp::And => builder.and(lhs_value, rhs_value),
                    BinaryOp::Or => builder.or(lhs_value, rhs_value),
                    BinaryOp::BitAnd => builder.bit_and(lhs_value, rhs_value),
                    BinaryOp::BitOr => builder.bit_or(lhs_value, rhs_value),
                    BinaryOp::Xor => builder.xor(lhs_value, rhs_value),
                    BinaryOp::Equal => builder.compare_eq(lhs_value, rhs_value),
                    BinaryOp::NotEqual => builder.compare_ne(lhs_value, rhs_value),
                    BinaryOp::Greater => builder.compare_gt(lhs_value, rhs_value),
                    BinaryOp::Less => builder.compare_lt(lhs_value, rhs_value),
                    BinaryOp::GreaterEqual => builder.compare_gte(lhs_value, rhs_value),
                    BinaryOp::LessEqual => builder.compare_lte(lhs_value, rhs_value),
                };

                Some(CodegenValue::RValue { value })
            }
            Expression::Unary { op, expr } => {
                let right =
                    Self::emit_expression(variables, types, builder, expr).unwrap();
                let right_value = right.extract(builder);

                let value = match op {
                    crate::ast::UnaryOp::Neg => builder.neg(right_value),
                    crate::ast::UnaryOp::Not => builder.not(right_value),
                    crate::ast::UnaryOp::Ref => todo!(),
                    crate::ast::UnaryOp::Deref => todo!(),
                };

                Some(CodegenValue::RValue { value })
            }
            Expression::Call { func, args } => {
                todo!();
            }
            Expression::Identifier { name } => {
                let sym = variables.get(name).unwrap();

                match sym {
                    Symbol::Function { .. } => todo!(),
                    Symbol::Value(v) => Some(v.clone()),
                }
            }
            Expression::Subscript { object, index } => {
                todo!();
            }
            Expression::Literal(literal) => {
                let value =
                    builder.alloc_constant(ir::constant::ConstantValue::Integer {
                        ty: types.get(&Ty::Builtin(crate::ast::Builtin::U32)).unwrap(),
                        value: *literal,
                    });
                Some(CodegenValue::RValue { value })
            }
            Expression::Range { from, to } => {
                //
                todo!();
            }
        }
    }
}
