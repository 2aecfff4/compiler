///
#[derive(Debug, Clone, Copy)]
pub enum Integer {
    U1,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl Integer {
    /// Provides the natural alignment of the type.
    pub fn alignment(&self) -> usize {
        match self {
            Integer::U1 => 1,
            Integer::U8 => 1,
            Integer::U16 => 2,
            Integer::U32 => 4,
            Integer::U64 => 8,
            Integer::I8 => 1,
            Integer::I16 => 2,
            Integer::I32 => 4,
            Integer::I64 => 8,
        }
    }

    /// Provides the size of the type.
    pub fn size_of(&self) -> Option<usize> {
        match self {
            Integer::U1 => None,
            Integer::U8 => Some(1),
            Integer::U16 => Some(2),
            Integer::U32 => Some(4),
            Integer::U64 => Some(8),
            Integer::I8 => Some(1),
            Integer::I16 => Some(2),
            Integer::I32 => Some(4),
            Integer::I64 => Some(8),
        }
    }
}

///
#[derive(Debug, Clone, Copy)]
pub enum Float {
    F32,
    F64,
}

impl Float {
    /// Provides the natural alignment of the type.
    pub fn alignment(&self) -> usize {
        match self {
            Float::F32 => 4,
            Float::F64 => 8,
        }
    }

    /// Provides the size of the type.
    pub fn size_of(&self) -> usize {
        match self {
            Float::F32 => 4,
            Float::F64 => 8,
        }
    }
}

///
#[derive(Debug, Clone)]
pub enum TypeKind {
    Integer { kind: Integer },
    Float { kind: Float },
    // #TODO:
    Struct { members: Vec<TypeKind> },
}

impl TypeKind {
    /// Provides the natural alignment of the type.
    pub fn alignment(&self) -> usize {
        match &self {
            TypeKind::Integer { kind } => kind.alignment(),
            TypeKind::Float { kind } => kind.alignment(),
            TypeKind::Struct { members } => {
                members //
                    .iter()
                    .map(|ty| ty.alignment())
                    .max()
                    .unwrap_or(1)
            }
        }
    }

    /// Provides the size of the type.
    pub fn size_of(&self) -> usize {
        match &self {
            TypeKind::Integer { kind } => kind.size_of().unwrap(),
            TypeKind::Float { kind } => kind.size_of(),
            TypeKind::Struct { members } => {
                let mut size = 0;

                for member in members.iter() {
                    let member_alignment = member.alignment();
                    if (size % member_alignment) != 0 {
                        size += member_alignment - (size % member_alignment);
                    }

                    size += member.size_of();
                }

                size
            }
        }
    }

    /// Provides the stride of the type.
    pub fn stride_of(&self) -> usize {
        match &self {
            TypeKind::Integer { kind } => kind.size_of().unwrap(),
            TypeKind::Float { kind } => kind.size_of(),
            TypeKind::Struct { members } => {
                let mut size: usize = 0;
                let mut alignment = 1;

                for member in members.iter() {
                    let member_alignment = member.alignment();
                    if (size % member_alignment) != 0 {
                        size += member_alignment - (size % member_alignment);
                    }

                    size += member.size_of();
                    alignment = alignment.max(member_alignment);
                }

                if (size % alignment) != 0 {
                    size += alignment - (size % alignment);
                }

                size
            }
        }
    }
}

///
#[derive(Debug, Clone)]
pub struct Type {
    kind: TypeKind,
    /// User-defined alignment must be greater than or equal to the type's natural alignment.
    alignment: Option<usize>,
    indirection: usize,
}

impl Type {
    /// # Parameters
    /// - alignment - User-defined alignment must be greater than or equal to the type's natural alignment.
    pub fn new(kind: TypeKind, alignment: Option<usize>) -> Self {
        Self {
            kind,
            alignment,
            indirection: 0,
        }
    }

    /// # Parameters
    /// - alignment - User-defined alignment must be greater than or equal to the type's natural alignment.
    pub fn new_ptr(kind: TypeKind, pointer_alignment: Option<usize>) -> Self {
        Self {
            kind,
            alignment: pointer_alignment,
            indirection: 1,
        }
    }

    ///
    pub fn to_ptr(self) -> Self {
        Self {
            kind: self.kind,
            alignment: self.alignment,
            indirection: self.indirection + 1,
        }
    }

    pub(crate) fn kind(&self) -> &TypeKind {
        &self.kind
    }
}
