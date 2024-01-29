use smallvec::{smallvec, SmallVec};

use crate::{function::Function, label::Label, ty::Type, value::Value};

///
#[derive(Debug, Clone)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    Shr,
    Shl,
    Sar,
    And,
    Or,
    Xor,
}

///
#[derive(Debug, Clone)]
pub(crate) enum UnaryOp {
    Neg,
    Not,
}

///
#[derive(Debug, Clone)]
pub(crate) enum CastOp {
    BitCast,
    SignExtend,
    Truncate,
    ZeroExtend,
}

///
#[derive(Debug, Clone)]
pub(crate) enum IntCompareOp {
    Equal,
    NotEqual,

    GreaterThan,
    GreaterThanOrEqual,

    LessThan,
    LessThanOrEqual,
}

///
#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    ArithmeticBinary {
        dst: Value,
        lhs: Value,
        op: BinaryOp,
        rhs: Value,
    },
    ArithmeticUnary {
        dst: Value,
        op: UnaryOp,
        value: Value,
    },
    Branch {
        target: Label,
    },
    BranchConditional {
        condition: Value,
        on_true: Label,
        on_false: Label,
    },
    Call {
        /// Handle to the function being called.
        function: Function,
        /// Arguments passed to the function.
        arguments: Vec<Value>,
        /// Values where the function return values are going to be stored.
        dst: Vec<Value>,
    },
    Cast {
        cast_op: CastOp,
        to_type: Type,
        dst: Value,
        value: Value,
    },
    GetElementPtr {
        dst: Value,
        ptr: Value,
        index: Value,
    },
    IntCompare {
        pred: IntCompareOp,
        dst: Value,
        lhs: Value,
        rhs: Value,
    },
    Load {
        dst: Value,
        ptr: Value,
    },
    Return {
        values: Option<Vec<Value>>,
    },
    Select {
        dst: Value,
        condition: Value,
        on_true: Value,
        on_false: Value,
    },
    StackAlloc {
        dst: Value,
        ty: Type,
        size: usize,
    },
    Store {
        ptr: Value,
        value: Value,
    },
}

impl Instruction {
    /// Retrieves the target labels associated with this instruction.
    /// Returns `None` for instructions without targets.
    pub fn targets(&self) -> Option<SmallVec<[Label; 8]>> {
        match self {
            Instruction::Return { .. } => Some(smallvec![]),
            Instruction::Branch { target } => Some(smallvec![*target]),
            Instruction::BranchConditional {
                on_true, on_false, ..
            } => Some(smallvec![*on_true, *on_false]),
            _ => None,
        }
    }
}
