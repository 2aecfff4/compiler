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
    BitAnd,
    BitOr,
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
        dst: Option<Value>,
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
        value: Option<Value>,
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
    Nop,
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

    ///
    pub fn creates(&self) -> Option<Value> {
        match self {
            Instruction::ArithmeticBinary { dst, .. } => Some(*dst),
            Instruction::ArithmeticUnary { dst, .. } => Some(*dst),
            Instruction::Branch { .. } => None,
            Instruction::BranchConditional { .. } => None,
            Instruction::Call { dst, .. } => *dst,
            Instruction::Cast { dst, .. } => Some(*dst),
            Instruction::GetElementPtr { dst, .. } => Some(*dst),
            Instruction::IntCompare { dst, .. } => Some(*dst),
            Instruction::Load { dst, .. } => Some(*dst),
            Instruction::Return { .. } => None,
            Instruction::Select { dst, .. } => Some(*dst),
            Instruction::StackAlloc { dst, .. } => Some(*dst),
            Instruction::Store { .. } => None,
            Instruction::Nop => None,
        }
    }

    ///
    pub fn reads(&self) -> Option<SmallVec<[Value; 4]>> {
        match self {
            Instruction::ArithmeticBinary { lhs, rhs, .. } => Some(smallvec![*lhs, *rhs]),
            Instruction::ArithmeticUnary { value, .. } => Some(smallvec![*value]),
            Instruction::Branch { .. } => None,
            Instruction::BranchConditional { condition, .. } => {
                Some(smallvec![*condition])
            }
            Instruction::Call { arguments, .. } => {
                if arguments.is_empty() {
                    None
                } else {
                    Some(arguments.clone().into())
                }
            }
            Instruction::Cast { value, .. } => Some(smallvec![*value]),
            Instruction::GetElementPtr { ptr, index, .. } => {
                Some(smallvec![*ptr, *index])
            }
            Instruction::IntCompare { lhs, rhs, .. } => Some(smallvec![*lhs, *rhs]),
            Instruction::Load { ptr, .. } => Some(smallvec![*ptr]),
            Instruction::Return { value } => (*value).map(|v| smallvec![v]),
            Instruction::Select {
                condition,
                on_true,
                on_false,
                ..
            } => Some(smallvec![*condition, *on_true, *on_false]),
            Instruction::StackAlloc { .. } => None,
            Instruction::Store { ptr, value } => Some(smallvec![*ptr, *value]),
            Instruction::Nop => None,
        }
    }
}
