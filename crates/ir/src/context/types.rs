use crate::{handles::TypeHandle, ty::Type};

///
pub(super) struct Types {
    types: Vec<Type>,
}

impl Types {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn create(&mut self, ty: Type) -> TypeHandle {
        let index = self.types.len();
        self.types.push(ty);

        TypeHandle::new(index.try_into().unwrap())
    }

    pub fn get(&self, handle: TypeHandle) -> Option<&Type> {
        let index = handle.id();
        if index < self.types.len() {
            Some(&self.types[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &Type)> {
        self.types
            .iter()
            .enumerate()
            .map(|(id, ty)| (id as u32, ty))
    }
}
