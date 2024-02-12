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

    ///
    pub(crate) fn functions_mut(&mut self) -> &mut Functions {
        &mut self.functions
    }

    ///
    pub(crate) fn types(&self) -> &Types {
        &self.types
    }

    pub fn optimize(&mut self) {
        use crate::passes;
        let mut passes: [&mut dyn passes::Pass; 4] = [
            &mut passes::constant_folding::ConstantFoldingPass,
            &mut passes::simplify_cfg::SimplifyCfgPass,
            &mut passes::dead_code_elimination::DeadCodeEliminationPass,
            &mut passes::remove_noops::RemoveNoopsPass,
        ];

        for (_, function) in self.functions.iter_mut() {
            let mut ctx = crate::passes::FunctionContext::new(&self.types, function);
            for pass in passes.iter_mut() {
                pass.run(&mut ctx);
            }
        }
    }

    pub fn dump_ir(&self, path: &std::path::Path) -> std::io::Result<()> {
        use itertools::*;
        use std::io::prelude::*;

        let mut file = std::fs::File::create(path)?;

        writeln!(file, "digraph {{")?;
        writeln!(file, "graph [fontname = \"helvetica\"];")?;
        writeln!(file, "edge [fontname = \"helvetica\", fontsize=10];")?;
        writeln!(
            file,
            "node [shape=rectangle, fontname=\"helvetica\", fontsize=10];\n"
        )?;

        for (id, function) in self.functions.iter() {
            let formatter = IrFormatter::new(&self.types, function);
            let cfg = function.labels().cfg();

            cfg.bfs(|label| {
                write!(
                    file,
                    "{label} [label = < <table border=\"0\" cellpadding=\"0\"> <tr><td border=\"1\" align=\"center\">{label}</td></tr>"
                )
                .unwrap();
                let data = function.labels().get(label);
                for instruction in data.instructions.iter() {
                    let instr = format_instruction(&formatter, instruction);
                    write!(file, "<tr><td align=\"left\">{instr}</td></tr>").unwrap();
                }
                writeln!(file, "</table> > ]").unwrap();
            });

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

            writeln!(file, "subgraph function_{id}_graph {{")?;
            writeln!(
                file,
                "label = \"fn @{function_name}({args}) -> {return_type}\";"
            )?;

            cfg.bfs(|label| {
                for (from, to, edge) in cfg.outgoing(label) {
                    writeln!(file, "{from} -> {to} [ label=\"{edge:?}\" ]").unwrap();
                }
            });

            writeln!(file, "}}")?;
        }

        writeln!(file, "}}")?;
        Ok(())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
