use crate::{
    context::label_ctx::LabelContext, function::FunctionData, label::Label, ty::Types,
    value::Value,
};

//////////////////////////////////////////////////////////////////////////////////////////
// FunctionContext

///
pub struct FunctionContext<'a> {
    types: &'a mut Types,
    function: &'a mut FunctionData,
}

impl<'a> FunctionContext<'a> {
    ///
    pub(super) fn new(types: &'a mut Types, function: &'a mut FunctionData) -> Self {
        Self { types, function }
    }

    ///
    pub fn parameter(&self, index: usize) -> Value {
        self.function.parameters()[index]
    }

    ///
    pub fn create_label(&mut self, name: &str) -> Label {
        self.function.labels_mut().create(name)
    }

    ///
    pub fn with_label<Func, R>(&mut self, label: Label, func: Func) -> R
    where
        Func: FnOnce(LabelContext<'_>) -> R,
    {
        let label = self.function.labels.get_mut(label);
        let values = &mut self.function.values;

        func(LabelContext::new(self.types, values, label))
    }
}
