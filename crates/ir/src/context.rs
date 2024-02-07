use crate::{
    dump_ir::{format_instruction, IrFormatter},
    function::{Function, Functions},
    function_builder::FunctionBuilder,
    ty::{Type, TypeKind, Types},
    value::Value,
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
    pub fn create_type(&mut self, ty: TypeKind) -> Type {
        self.types.create(ty)
    }

    ///
    pub fn create_function(
        &mut self,
        name: &str,
        return_type: Option<Type>,
        parameter_types: &[Type],
    ) -> Function {
        self.functions.create(name, return_type, parameter_types)
    }

    ///
    pub fn builder(&mut self, function: Function) -> FunctionBuilder<'_> {
        FunctionBuilder::new(&mut self.types, self.functions.get_mut(function))
    }

    pub fn validate(&mut self) {
        // Ensure each label has only one branch, and it's the last instruction.
        for (_, function) in self.functions.iter() {
            for (_, label) in function.labels.iter() {
                let count = label
                    .instructions
                    .iter()
                    .filter(|instr| instr.targets().is_some())
                    .count();
                assert_eq!(count, 1);

                let last_instruction = label.instructions.last().unwrap();
                assert!(last_instruction.targets().is_some());
            }

        }
    }

    pub fn dump_ir(&self, path: &std::path::Path) {
        use std::io::prelude::*;
        let mut file = std::fs::File::create(path).unwrap();

        for (id, function) in self.functions.iter() {
            let formatter = IrFormatter::new(&self.types, function);
            let cfg = function.labels.cfg();

            use itertools::*;
            let definition = function.definition();

            let return_type = if let Some(ty) = definition.return_type {
                formatter.ty(ty)
            } else {
                "".to_string()
            };

            // #TODO:
            let args = function
                .parameters()
                .iter()
                .zip(definition.parameter_types.iter())
                .map(|(value, ty)| format!("v{}: {}", value.id(), formatter.ty(*ty)))
                .join(", ");

            let function_name = &function.definition().name;
            writeln!(file, "fn @{function_name}({args}) -> {return_type} {{").unwrap();

            for (label_id, label) in function.labels().iter() {
                writeln!(file, "    block_{}: {{", label_id).unwrap();

                for instruction in label.instructions.iter() {
                    write!(file, "        ").unwrap();
                    format_instruction(&mut file, &formatter, instruction).unwrap();
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
