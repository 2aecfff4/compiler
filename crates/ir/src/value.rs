use crate::{handle_impl, ty::Type};

handle_impl! {
    ///
    impl Value
}

///
pub(crate) struct ValueData {
    ty: Type,
}

impl ValueData {
    pub fn ty(&self) -> Type {
        self.ty
    }
}
///
pub(crate) struct Values {
    values: Vec<ValueData>,
}

impl Values {
    ///
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    ///
    pub fn alloc(&mut self, ty: Type) -> Value {
        let index = self.values.len();
        self.values.push(ValueData { ty });

        Value(index.try_into().unwrap())
    }

    ///
    pub fn get(&self, handle: Value) -> &ValueData {
        let index = handle.id();
        self.values.get(index).unwrap()
    }

    ///
    pub fn get_mut(&mut self, handle: Value) -> &mut ValueData {
        let index = handle.id();
        self.values.get_mut(index).unwrap()
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (Value, &ValueData)> {
        self.values
            .iter()
            .enumerate()
            .map(|(id, label)| (Value(id as u32), label))
    }
}
