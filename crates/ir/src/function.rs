use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    constant::{Constant, Constants},
    handle_impl,
    instruction::Instruction,
    label::{Label, Labels},
    location::Location,
    pointer_analysis::PointerAnalysis,
    ty::{Type, Types},
    value::{Value, Values},
};

handle_impl! {
    ///
    impl Function
}

//////////////////////////////////////////////////////////////////////////////////////////
// FunctionDefinition

///
pub(crate) struct FunctionDefinition {
    pub name: String,
    pub return_type: Option<Type>,
    /// Variables in a method definition.
    pub parameter_types: Vec<Type>,
}

//////////////////////////////////////////////////////////////////////////////////////////
// FunctionData

///
pub(crate) struct FunctionData {
    definition: FunctionDefinition,
    pub values: Values,
    pub labels: Labels,
    parameters: Vec<Value>,
    pub constants: Constants,
    pub value_to_constant: HashMap<Value, Constant>,
}

impl FunctionData {
    ///
    pub fn new(name: &str, return_type: Option<Type>, parameter_types: &[Type]) -> Self {
        let mut values = Values::new();
        let labels = Labels::new();
        let parameters = parameter_types //
            .iter()
            .map(|ty| values.alloc(*ty))
            .collect();
        let constants = Constants::new();

        let parameter_types = parameter_types.to_vec();
        let definition = FunctionDefinition {
            name: name.to_string(),
            return_type,
            parameter_types,
        };

        FunctionData {
            definition,
            values,
            labels,
            parameters,
            constants,
            value_to_constant: HashMap::new(),
        }
    }

    ///
    pub fn definition(&self) -> &FunctionDefinition {
        &self.definition
    }

    ///
    pub fn values(&self) -> &Values {
        &self.values
    }

    ///
    pub fn values_mut(&mut self) -> &mut Values {
        &mut self.values
    }

    ///
    pub fn labels(&self) -> &Labels {
        &self.labels
    }

    ///
    pub fn labels_mut(&mut self) -> &mut Labels {
        &mut self.labels
    }

    ///
    pub fn parameters(&self) -> &[Value] {
        &self.parameters
    }

    ///
    pub fn constants(&self) -> &Constants {
        &self.constants
    }

    ///
    pub fn instruction(&self, location: &Location) -> &Instruction {
        &self.labels().get(location.label).instructions[location.instruction as usize]
    }

    pub fn last_instruction(&self, label: Label) -> &Instruction {
        self.labels().get(label).instructions.last().unwrap()
    }

    ///
    pub fn instruction_mut(&mut self, location: &Location) -> &mut Instruction {
        &mut self //
            .labels_mut()
            .get_mut(location.label)
            .instructions[location.instruction as usize]
    }

    /// These are essentially outgoing edges.
    /// # Example
    /// IR
    /// ```
    /// let v0: *u32 = stack_alloc.u32 1
    ///      ▼__________________
    ///                         ▼
    /// let v1: u32 = load.*u32 v0
    /// ```
    /// Result
    /// ```
    /// {
    ///     "v0": { v1 },
    ///     "v1": {}
    /// }
    /// ```
    pub fn variable_users(&self) -> HashMap<Value, HashSet<Location>> {
        let mut edges: HashMap<Value, HashSet<Location>> = HashMap::new();
        for (value, _) in self.values().iter() {
            edges.entry(value).or_default();
        }

        let cfg = self.labels.cfg();
        cfg.bfs(|label| {
            let label_data = self.labels().get(label);
            for (i, instr) in label_data.instructions.iter().enumerate() {
                let location = Location {
                    label,
                    instruction: i as u32,
                };
                if let Some(reads) = instr.reads() {
                    for read in reads.iter() {
                        edges.entry(*read).or_default().insert(location);
                    }
                }
            }
        });

        edges
    }

    ///
    pub fn topological_sort(&self) -> Vec<Value> {
        // Kahn's algorithm
        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm

        let cfg = self.labels.cfg();

        let mut dependencies: HashMap<Value, HashSet<(Location, Value)>> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut in_degree = HashMap::new();

        let mut expected_value_count = 0;
        for parameter in self.parameters() {
            in_degree.entry(*parameter).or_insert(0);
            expected_value_count += 1;
        }

        for (k, _) in self.value_to_constant.iter() {
            in_degree.entry(*k).or_insert(0);
            expected_value_count += 1;
        }

        cfg.bfs(|label| {
            let label_data = self.labels().get(label);
            for (i, instr) in label_data.instructions.iter().enumerate() {
                let location = Location {
                    label,
                    instruction: i as u32,
                };
                if let Some(creates) = instr.creates() {
                    expected_value_count += 1;
                    in_degree.entry(creates).or_insert(0);
                    if let Some(reads) = instr.reads() {
                        for read in reads.iter() {
                            dependencies
                                .entry(*read)
                                .or_default()
                                .insert((location, creates));

                            in_degree.entry(creates).and_modify(|degree| *degree += 1);
                        }
                    }
                }
            }
        });

        for (value, degree) in in_degree.iter() {
            if *degree == 0 {
                queue.push_back(*value);
            }
        }

        let mut result = Vec::with_capacity(expected_value_count);

        while let Some(value) = queue.pop_front() {
            result.push(value);

            if let Some(users) = dependencies.get(&value) {
                for (_, creator) in users.iter() {
                    let degree = in_degree.get_mut(creator).unwrap();
                    *degree -= 1;
                    assert!(*degree >= 0);
                    if *degree == 0 {
                        queue.push_back(*creator);
                    }
                }
            }
        }

        result
    }

    ///
    pub fn pointer_analysis(&self, types: &Types) -> PointerAnalysis {
        PointerAnalysis::new(types, self, &self.labels)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Functions

///
pub(crate) struct Functions {
    functions: Vec<FunctionData>,
}

impl Functions {
    ///
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    ///
    pub fn create(
        &mut self,
        name: &str,
        return_type: Option<Type>,
        parameter_types: &[Type],
    ) -> Function {
        let index = self.functions.len();
        self.functions
            .push(FunctionData::new(name, return_type, parameter_types));

        Function(index.try_into().unwrap())
    }

    ///
    pub fn get(&self, handle: Function) -> &FunctionData {
        let index = handle.id();
        self.functions.get(index).unwrap()
    }

    ///
    pub fn get_mut(&mut self, handle: Function) -> &mut FunctionData {
        let index = handle.id();
        self.functions.get_mut(index).unwrap()
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (u32, &FunctionData)> {
        self.functions
            .iter()
            .enumerate()
            .map(|(id, label)| (id as u32, label))
    }

    ///
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut FunctionData)> {
        self.functions
            .iter_mut()
            .enumerate()
            .map(|(id, label)| (id as u32, label))
    }
}
