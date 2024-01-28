use std::collections::HashSet;

use crate::{
    context::{
        label::{LabelContext, Labels},
        types::Types,
        values::Values,
    },
    handles::{FunctionHandle, LabelHandle, TypeHandle, ValueHandle},
    ty::TypeKind,
};

//////////////////////////////////////////////////////////////////////////////////////////
// FunctionDefinition

///
pub(super) struct FunctionDefinition {
    pub name: String,
    pub return_types: Option<Vec<TypeHandle>>,
    /// Variables in a method definition.
    pub parameter_types: Vec<TypeHandle>,
}

//////////////////////////////////////////////////////////////////////////////////////////
// Function

///
pub(super) struct Function {
    definition: FunctionDefinition,
    values: Values,
    labels: Labels,
    parameters: Vec<ValueHandle>,
    // constants:
}

impl Function {
    pub fn new(
        name: &str,
        return_types: Option<&[TypeHandle]>,
        parameter_types: &[TypeHandle],
    ) -> Self {
        let mut values = Values::new();
        // #TODO: Values should be typed?
        let parameters_values = parameter_types.iter().map(|ty| values.alloc()).collect();

        let return_types = return_types.map(|types| types.to_vec());
        let parameter_types = parameter_types.to_vec();
        let definition = FunctionDefinition {
            name: name.to_string(),
            return_types,
            parameter_types,
        };

        Function {
            definition,
            values,
            labels: Labels::new(),
            parameters: parameters_values,
        }
    }

    pub fn definition(&self) -> &FunctionDefinition {
        &self.definition
    }

    pub fn values(&self) -> &Values {
        &self.values
    }

    pub fn labels(&self) -> &Labels {
        &self.labels
    }

    pub fn parameters(&self) -> &[ValueHandle] {
        &self.parameters
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// FunctionContext

///
pub struct FunctionContext<'a> {
    types: &'a Types,
    function: &'a mut Function,
}

impl<'a> FunctionContext<'a> {
    ///
    pub(super) fn new(types: &'a Types, function: &'a mut Function) -> Self {
        Self { types, function }
    }

    ///
    pub fn parameter(&self, index: usize) -> ValueHandle {
        self.function.parameters[index]
    }

    ///
    pub fn create_label(&mut self, name: &str) -> LabelHandle {
        self.function.labels.create(name)
    }

    ///
    pub fn with_label<Func, R>(&mut self, label: LabelHandle, func: Func) -> R
    where
        Func: FnOnce(LabelContext<'_>) -> R,
    {
        let label = self.function.labels.get_mut(label).unwrap();
        let values = &mut self.function.values;

        func(LabelContext::new(self.types, values, label))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Functions

///
pub(super) struct Functions {
    functions: Vec<Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn create(
        &mut self,
        name: &str,
        return_types: Option<&[TypeHandle]>,
        parameter_types: &[TypeHandle],
    ) -> FunctionHandle {
        let index = self.functions.len();
        self.functions
            .push(Function::new(name, return_types, parameter_types));

        FunctionHandle::new(index.try_into().unwrap())
    }

    pub fn get(&self, handle: FunctionHandle) -> Option<&Function> {
        let index = handle.id();
        if index < self.functions.len() {
            Some(&self.functions[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, handle: FunctionHandle) -> Option<&mut Function> {
        let index = handle.id();
        if index < self.functions.len() {
            Some(&mut self.functions[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &Function)> {
        self.functions
            .iter()
            .enumerate()
            .map(|(id, ty)| (id as u32, ty))
    }
}
