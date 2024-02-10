use crate::{
    cfg::Cfg, function::FunctionData, instruction::Instruction, label::Labels,
    location::Location, ty::Types, value::Value,
};
use std::collections::{HashMap, HashSet};

///
#[derive(Debug)]
pub(crate) struct PointerAnalysis {
    pub creators: HashMap<Value, Location>,
    pub pointers: HashSet<Value>,
    pub pointer_origins: HashMap<Value, Value>,
    pub escaped_pointers: HashSet<Value>,
}

impl PointerAnalysis {
    ///
    pub fn new(types: &Types, function: &FunctionData, labels: &Labels) -> Self {
        let cfg = labels.cfg();

        let creators = Self::compute_creators(labels, &cfg);
        let topologically_sorted_values = function.topological_sort();
        let (pointers, pointer_origins) = Self::compute_pointer_origins(
            types,
            function,
            &creators,
            &topologically_sorted_values,
        );
        let escaped_pointers =
            Self::escape_analysis(function, &topologically_sorted_values, &pointers);

        PointerAnalysis {
            creators,
            pointers,
            pointer_origins,
            escaped_pointers,
        }
    }

    ///
    fn compute_creators(labels: &Labels, cfg: &Cfg) -> HashMap<Value, Location> {
        let mut res = HashMap::new();

        cfg.bfs(|label| {
            let label_data = labels.get(label);

            for (i, instr) in label_data.instructions.iter().enumerate() {
                if let Some(value) = instr.creates() {
                    let other = res.insert(
                        value,
                        Location {
                            label,
                            instruction: i as u32,
                        },
                    );
                    assert!(other.is_none());
                }
            }
        });

        res
    }

    ///
    fn compute_pointer_origins(
        types: &Types,
        function: &FunctionData,
        creators: &HashMap<Value, Location>,
        topologically_sorted_values: &[Value],
    ) -> (HashSet<Value>, HashMap<Value, Value>) {
        let mut pointers = HashSet::new();
        let mut pointer_origins = HashMap::new();

        for value in topologically_sorted_values.iter() {
            if !types.is_pointer(function.values().get(*value).ty()) {
                continue;
            }
            pointers.insert(*value);

            let origin = if let Some(creator) = creators.get(value) {
                match function.instruction(creator) {
                    Instruction::Call { .. } => value,
                    Instruction::Cast { .. } => value,
                    Instruction::GetElementPtr { ptr, .. } => {
                        pointer_origins.get(ptr).unwrap()
                    }
                    Instruction::Load { .. } => value,
                    Instruction::Select {
                        on_true, on_false, ..
                    } => {
                        let on_true_origin = pointer_origins.get(on_true).unwrap();
                        let on_false_origin = pointer_origins.get(on_false).unwrap();

                        if on_true_origin == on_false_origin {
                            on_true_origin
                        } else {
                            // In this case we don't have a common origin
                            value
                        }
                    }
                    Instruction::StackAlloc { .. } => value,
                    other => panic!("{other:?}"),
                }
            } else {
                // This could be a constant or a function parameter.
                value
            };

            let already_exist = pointer_origins.insert(*value, *origin).is_some();
            assert!(!already_exist);
        }

        (pointers, pointer_origins)
    }

    ///
    fn escape_analysis(
        function: &FunctionData,
        topologically_sorted_values: &[Value],
        pointers: &HashSet<Value>,
    ) -> HashSet<Value> {
        let mut escaped_pointers = HashSet::new();
        let outgoing_edges = function.variable_users();

        // Catch pointers that are escaping
        for pointer in topologically_sorted_values.iter().rev() {
            if !pointers.contains(pointer) {
                continue;
            }

            let mut escaped = false;
            let neighbors = outgoing_edges.get(pointer).unwrap();
            for location in neighbors.iter() {
                let instr = function.instruction(location);
                escaped = match instr {
                    // #TODO: Arithmetic instructions are fine,
                    // as long as they are not doing any pointer arithmetic?
                    Instruction::ArithmeticBinary { .. } => true,
                    Instruction::ArithmeticUnary { .. } => true,
                    Instruction::Branch { .. } => panic!(),
                    Instruction::BranchConditional { .. } => panic!(),
                    Instruction::Call { .. } => true,
                    Instruction::Cast { .. } => true,
                    Instruction::GetElementPtr { dst, ptr, .. } => {
                        *ptr != *pointer && !escaped_pointers.contains(dst)
                    }
                    Instruction::IntCompare { .. } => false,
                    Instruction::Load { .. } => false,
                    Instruction::Return { .. } => false,
                    Instruction::Select { .. } => true,
                    Instruction::StackAlloc { .. } => panic!(),
                    Instruction::Store { ptr, value } => {
                        *ptr != *pointer && *value == *pointer
                    }
                    Instruction::Nop => panic!(),
                };

                if escaped {
                    break;
                }
            }

            if escaped {
                escaped_pointers.insert(*pointer);
            }
        }

        escaped_pointers
    }
}
