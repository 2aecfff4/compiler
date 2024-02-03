use crate::{handle_impl, ty::Type};

handle_impl! {
    ///
    impl Constant
}

///
pub enum ConstantValue {
    Integer { ty: Type, value: u64 },
}

///
pub(crate) struct ConstantData {}

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
    pub fn create(&mut self) -> Constant {
        let index = self.constants.len();
        self.constants.push(ConstantData {});

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
