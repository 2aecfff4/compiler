use std::collections::HashMap;

use crate::{
    constant::{Constant, Constants},
    handle_impl,
    label::Labels,
    ty::Type,
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
}
