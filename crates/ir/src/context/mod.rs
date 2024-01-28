pub(super) mod function;
pub(super) mod label;
pub(super) mod types;
pub(super) mod values;

use crate::{
    context::{
        function::{Function, FunctionContext, Functions},
        types::Types,
    },
    handles::{FunctionHandle, TypeHandle},
    ty::{Type, TypeKind},
};

///
pub struct Context {
    types: Types,
    functions: Functions,
}

impl Context {
    ///
    pub fn new() -> Self {
        Self {
            types: Types::new(),
            functions: Functions::new(),
        }
    }

    ///
    pub fn create_type(&mut self, ty: Type) -> TypeHandle {
        self.types.create(ty)
    }

    ///
    pub fn create_function(
        &mut self,
        name: &str,
        return_types: Option<&[TypeHandle]>,
        parameter_types: &[TypeHandle],
    ) -> FunctionHandle {
        self.functions.create(name, return_types, parameter_types)
    }

    ///
    pub fn with_function<Func, R>(&mut self, function: FunctionHandle, func: Func) -> R
    where
        Func: FnOnce(FunctionContext<'_>) -> R,
    {
        let types = &self.types;
        let function = self.functions.get_mut(function).unwrap();

        func(FunctionContext::new(types, function))
    }

    /// #TODO: Implement this properly
    pub fn dump_ir(&self, path: &std::path::Path) {
        use std::io::prelude::*;
        let mut file = std::fs::File::create(path).unwrap();

        for (id, ty) in self.types.iter() {
            let definition = match ty.kind() {
                TypeKind::Integer { kind } => {
                    use crate::ty::Integer;
                    match kind {
                        Integer::U1 => "u1",
                        Integer::U8 => "u8",
                        Integer::U16 => "u16",
                        Integer::U32 => "u32",
                        Integer::U64 => "u64",
                        Integer::I8 => "i8",
                        Integer::I16 => "i16",
                        Integer::I32 => "i32",
                        Integer::I64 => "i64",
                    }
                }
                TypeKind::Float { kind } => {
                    use crate::ty::Float;
                    match kind {
                        Float::F32 => "f32",
                        Float::F64 => "f64",
                    }
                }
                TypeKind::Struct { members } => todo!(),
            };

            writeln!(file, "t{id} = type {{ {definition} }}").unwrap();
        }

        writeln!(file).unwrap();

        for (id, function) in self.functions.iter() {
            use itertools::*;
            let definition = function.definition();

            let return_types = {
                if let Some(ref return_types) = definition.return_types {
                    return_types
                        .iter()
                        .map(|ty| format!("t{}", ty.id()))
                        .join(", ")
                } else {
                    "".to_string()
                }
            };

            // #TODO:
            let args = function
                .parameters()
                .iter()
                .zip(definition.parameter_types.iter())
                .map(|(value, ty)| format!("v{}: t{}", value.id(), ty.id()))
                .join(", ");

            let function_name = &function.definition().name;
            writeln!(file, "fn @{function_name}({args}) -> ({return_types}) {{").unwrap();

            for (_, label) in function.labels().iter() {
                writeln!(file, "    {}: {{", label.name).unwrap();

                for instruction in label.instructions.iter() {
                    let i = match instruction {
                        crate::instruction::Instruction::ArithmeticBinary {
                            dst,
                            lhs,
                            op,
                            rhs,
                        } => {
                            format!("v{} = {} v{}, v{}", dst.id(), op, lhs.id(), rhs.id())
                        }
                        crate::instruction::Instruction::ArithmeticUnary {
                            dst,
                            op,
                            value,
                        } => format!("v{} = {} v{}", dst.id(), op, value.id()),
                        crate::instruction::Instruction::Branch { target } => {
                            let label = function.labels().get(*target).unwrap();
                            let name = &label.name;
                            format!("branch {name}")
                        }
                        crate::instruction::Instruction::BranchConditional {
                            condition,
                            on_true,
                            on_false,
                        } => {
                            let on_true = &function.labels().get(*on_true).unwrap().name;
                            let on_false =
                                &function.labels().get(*on_false).unwrap().name;
                            format!(
                                "branch_if v{}, {}, {}",
                                condition.id(),
                                on_true,
                                on_false
                            )
                        }
                        crate::instruction::Instruction::Call {
                            function,
                            arguments,
                            dst,
                        } => todo!(),
                        crate::instruction::Instruction::Cast {
                            cast_op,
                            dst,
                            value,
                            ..
                        } => format!("v{} = {} v{}", dst.id(), cast_op, value.id()),
                        crate::instruction::Instruction::GetElementPtr {
                            dst,
                            source,
                            index,
                        } => format!(
                            "v{} = get_element_ptr v{}, v{}",
                            dst.id(),
                            source.id(),
                            index.id()
                        ),
                        crate::instruction::Instruction::IntCompare {
                            dst,
                            lhs,
                            rhs,
                            pred,
                        } => format!(
                            "v{} = {} v{}, v{}",
                            dst.id(),
                            pred,
                            lhs.id(),
                            rhs.id()
                        ),
                        crate::instruction::Instruction::Load { dst, ptr } => {
                            format!("v{} = load v{}", dst.id(), ptr.id())
                        }
                        crate::instruction::Instruction::Return { values } => {
                            let values = {
                                if let Some(values) = values {
                                    values
                                        .iter()
                                        .map(|value| format!("v{}", value.id()))
                                        .join(", ")
                                } else {
                                    "".to_string()
                                }
                            };
                            format!("ret ({values})")
                        }
                        crate::instruction::Instruction::Select {
                            dst,
                            condition,
                            on_true,
                            on_false,
                        } => format!(
                            "v{} = select v{}, v{}, v{}",
                            dst.id(),
                            condition.id(),
                            on_true.id(),
                            on_false.id()
                        ),
                        crate::instruction::Instruction::StackAlloc { dst, size, ty } => {
                            todo!()
                        }
                        crate::instruction::Instruction::Store { ptr, value } => {
                            format!("store v{}, v{}", ptr.id(), value.id())
                        }
                    };
                    //writeln!(file, "        {instruction:?}").unwrap();
                    writeln!(file, "        {i}").unwrap();
                }

                writeln!(file, "    }}").unwrap();
            }

            writeln!(file, "}}\n").unwrap();
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
