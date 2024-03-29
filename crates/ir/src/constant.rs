use crate::{handle_impl, ty::Type};

handle_impl! {
    ///
    impl Constant
}

///
#[derive(Debug, Clone, Copy)]
pub enum ConstantValue {
    Integer { ty: Type, value: u64 },
    Float { ty: Type, value: f64 },
}

impl ConstantValue {
    pub fn ty(&self) -> Type {
        match self {
            ConstantValue::Integer { ty, value } => *ty,
            ConstantValue::Float { ty, value } => *ty,
        }
    }

    ///
    pub fn integer(&self) -> Option<u64> {
        match self {
            ConstantValue::Integer { value, .. } => Some(*value),
            _ => None,
        }
    }
}

///
pub(crate) struct ConstantData {
    pub value: ConstantValue,
}

//////////////////////////////////////////////////////////////////////////////////////////
// Labels

///
pub(crate) struct Constants {
    constants: Vec<ConstantData>,
}

impl Constants {
    ///
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
        }
    }

    ///
    pub fn create(&mut self, value: ConstantValue) -> Constant {
        let index = self.constants.len();
        self.constants.push(ConstantData { value });

        Constant(index.try_into().unwrap())
    }

    ///
    pub fn get(&self, handle: Constant) -> &ConstantData {
        let index = handle.id();
        self.constants.get(index).unwrap()
    }

    ///
    pub fn get_mut(&mut self, handle: Constant) -> &mut ConstantData {
        let index = handle.id();
        self.constants.get_mut(index).unwrap()
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (u32, &ConstantData)> {
        self.constants
            .iter()
            .enumerate()
            .map(|(id, label)| (id as u32, label))
    }
}
