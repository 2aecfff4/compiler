use crate::handle_impl;

handle_impl! {
    ///
    impl Type
}

///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeKind {
    Integer { num_bits: u32, is_signed: bool },
    Float { num_bits: u32 },
    Struct { types: Vec<Type> },
    Pointer { ty: Type },
}

///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct TypeData {
    ty: TypeKind,
}

impl TypeData {
    pub fn ty(&self) -> &TypeKind {
        &self.ty
    }
}

///
pub(crate) struct Types {
    types: Vec<TypeData>,
}

impl Types {
    ///
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    ///
    pub fn create(&mut self, ty: TypeKind) -> Type {
        let index = self.types.len();
        self.types.push(TypeData { ty });

        Type(index.try_into().unwrap())
    }

    ///
    pub fn types_match(&self, a: Type, b: Type) -> bool {
        let a = self.get(a);
        let b = self.get(b);

        // #TODO
        println!("{a:?} == {b:?}");

        a == b
    }

    ///
    pub fn strip_pointer(&self, handle: Type) -> Option<Type> {
        let type_data = self.get(handle);

        match type_data.ty {
            TypeKind::Pointer { ty } => Some(ty),
            _ => None,
        }
    }

    ///
    pub fn add_pointer(&mut self, handle: Type) -> Type {
        self.create(TypeKind::Pointer { ty: handle })
    }

    ///
    pub fn is_pointer(&self, handle: Type) -> bool {
        let type_data = self.get(handle);
        matches!(type_data.ty, TypeKind::Pointer { .. })
    }

    ///
    pub fn is_arithmetic(&self, handle: Type) -> bool {
        let type_data = self.get(handle);
        matches!(type_data.ty, TypeKind::Integer { .. })
    }

    ///
    pub fn is_struct(&self, handle: Type) -> bool {
        let type_data = self.get(handle);
        matches!(type_data.ty, TypeKind::Struct { .. })
    }

    ///
    pub fn get(&self, handle: Type) -> &TypeData {
        let index = handle.id();
        self.types.get(index).unwrap()
    }

    ///
    pub fn get_mut(&mut self, handle: Type) -> &mut TypeData {
        let index = handle.id();
        self.types.get_mut(index).unwrap()
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (u32, &TypeData)> {
        self.types
            .iter()
            .enumerate()
            .map(|(id, ty)| (id as u32, ty))
    }
}
