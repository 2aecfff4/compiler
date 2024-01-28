use crate::handles::{FunctionHandle, LabelHandle, TypeHandle, ValueHandle};

///
#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
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
#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum UnaryOp {
    Neg,
    Not,
}

///
#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
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

impl std::fmt::Display for IntCompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntCompareOp::Equal => write!(f, "eq"),
            IntCompareOp::NotEqual => write!(f, "neq"),
            IntCompareOp::GreaterThan => write!(f, "gt"),
            IntCompareOp::GreaterThanOrEqual => write!(f, "gte"),
            IntCompareOp::LessThan => write!(f, "lt"),
            IntCompareOp::LessThanOrEqual => write!(f, "lte"),
        }
    }
}

///
#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    ArithmeticBinary {
        dst: ValueHandle,
        lhs: ValueHandle,
        op: BinaryOp,
        rhs: ValueHandle,
    },
    ArithmeticUnary {
        dst: ValueHandle,
        op: UnaryOp,
        value: ValueHandle,
    },
    Branch {
        target: LabelHandle,
    },
    BranchConditional {
        condition: ValueHandle,
        on_true: LabelHandle,
        on_false: LabelHandle,
    },
    Call {
        /// Handle to the function being called.
        function: FunctionHandle,
        /// Arguments passed to the function.
        arguments: Vec<ValueHandle>,
        /// Values where the function return values are going to be stored.
        dst: Vec<ValueHandle>,
    },
    Cast {
        cast_op: CastOp,
        to_type: TypeHandle,
        dst: ValueHandle,
        value: ValueHandle,
    },
    GetElementPtr {
        dst: ValueHandle,
        source: ValueHandle,
        index: ValueHandle,
    },
    IntCompare {
        dst: ValueHandle,
        lhs: ValueHandle,
        rhs: ValueHandle,
        pred: IntCompareOp,
    },
    Load {
        dst: ValueHandle,
        ptr: ValueHandle,
    },
    Return {
        values: Option<Vec<ValueHandle>>,
    },
    Select {
        dst: ValueHandle,
        condition: ValueHandle,
        on_true: ValueHandle,
        on_false: ValueHandle,
    },
    StackAlloc {
        dst: ValueHandle,
        size: usize,
        ty: TypeHandle,
    },
    Store {
        ptr: ValueHandle,
        value: ValueHandle,
    },
}
