///

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    Ref,
    Deref,
}

///

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    Shr,
    Shl,
    And,
    Or,
    BitAnd,
    BitOr,
    Xor,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

///

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssignOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Xor,
    Or,
    Not,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    Identifier {
        name: String,
    },
    Subscript {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Literal(u64),
    Range {
        from: Box<Expression>,
        to: Box<Expression>,
    },
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParamRef {
    Val,
    Ref,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub param_ref: ParamRef,
    pub ty: Box<Ty>,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: Box<Ty>,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Builtin {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Builtin(Builtin),
    NamedType {
        name: String,
    },
    Array {
        ty: Box<Ty>,
        size: usize,
    },
    Tuple {
        types: Vec<Ty>,
    },
    Function {
        ret: Box<Ty>,
        params: Vec<FunctionParam>,
    },
    Struct {
        fields: Vec<StructField>,
    },
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Block {
        nodes: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
        on_true: Box<Statement>,
        on_false: Option<Box<Statement>>,
    },
    For {
        value: Box<Expression>,
        range: Box<Expression>,
        body: Box<Statement>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    Return {
        expr: Option<Box<Expression>>,
    },
    Break,
    Continue,

    // Assignments
    /// a = b
    Assign {
        dst: Box<Expression>,
        src: Box<Expression>,
    },
    /// a += b
    CompoundAssign {
        dst: Box<Expression>,
        op: AssignOp,
        src: Box<Expression>,
    },

    // Declarations
    Function {
        name: String,
        params: Vec<FunctionParam>,
        ret: Box<Ty>,
        body: Box<Statement>,
    },
    Struct {},
    Let {
        name: String,
        ty: Ty,
        expr: Box<Expression>,
    },
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Declaration {}
