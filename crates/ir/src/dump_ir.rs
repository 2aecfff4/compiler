use crate::{
    constant::ConstantValue,
    context::Context,
    function::FunctionData,
    instruction::{BinaryOp, CastOp, Instruction, IntCompareOp, UnaryOp},
    label::Label,
    ty::{Type, Types},
    value::Value,
};
use std::{
    fmt::{self, Display},
    io::{self, Write},
};

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.id())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block_{}", self.id())
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "add"),
            BinaryOp::Sub => write!(f, "sub"),
            BinaryOp::Mul => write!(f, "mul"),
            BinaryOp::Mod => write!(f, "mod"),
            BinaryOp::Div => write!(f, "div"),
            BinaryOp::Shr => write!(f, "shr"),
            BinaryOp::Shl => write!(f, "shl"),
            BinaryOp::Sar => write!(f, "sar"),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::Or => write!(f, "or"),
            BinaryOp::Xor => write!(f, "xor"),
            BinaryOp::BitAnd => write!(f, "bit_and"),
            BinaryOp::BitOr => write!(f, "bit_or"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "neg"),
            UnaryOp::Not => write!(f, "not"),
        }
    }
}

impl fmt::Display for CastOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastOp::BitCast => write!(f, "bit_cast"),
            CastOp::SignExtend => write!(f, "sign_extend"),
            CastOp::Truncate => write!(f, "truncate"),
            CastOp::ZeroExtend => write!(f, "zero_extend"),
        }
    }
}

impl fmt::Display for IntCompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub(crate) struct IrFormatter<'a> {
    types: &'a Types,
    function_data: &'a FunctionData,
}

impl<'a> IrFormatter<'a> {
    pub fn new(types: &'a Types, function_data: &'a FunctionData) -> Self {
        Self {
            types,
            function_data,
        }
    }

    pub fn value_type(&self, value: Value) -> String {
        let value_data = self.function_data.values().get(value);
        let ty = value_data.ty();
        let mut type_data = self.types.get(ty);
        let mut type_kind = type_data.ty();

        let mut indirection = 0;

        while let crate::ty::TypeKind::Pointer { ty } = type_kind {
            indirection += 1;
            type_data = self.types.get(*ty);
            type_kind = type_data.ty();
        }

        match type_kind {
            crate::ty::TypeKind::Integer {
                num_bits,
                is_signed,
            } => {
                let ptr = "*".repeat(indirection);
                if *is_signed {
                    format!("{ptr}i{num_bits}")
                } else {
                    format!("{ptr}u{num_bits}")
                }
            }
            crate::ty::TypeKind::Float { num_bits } => {
                let ptr = "*".repeat(indirection);
                format!("{ptr}f{num_bits}")
            }
            _ => panic!(),
        }
    }

    pub fn ty(&self, handle: Type) -> String {
        let mut type_data = self.types.get(handle);
        let mut type_kind = type_data.ty();

        let mut indirection = 0;

        while let crate::ty::TypeKind::Pointer { ty } = type_kind {
            indirection += 1;
            type_data = self.types.get(*ty);
            type_kind = type_data.ty();
        }

        match type_kind {
            crate::ty::TypeKind::Integer {
                num_bits,
                is_signed,
            } => {
                let ptr = "*".repeat(indirection);
                if *is_signed {
                    format!("{ptr}i{num_bits}")
                } else {
                    format!("{ptr}u{num_bits}")
                }
            }
            crate::ty::TypeKind::Float { num_bits } => {
                let ptr = "*".repeat(indirection);
                format!("{ptr}f{num_bits}")
            }
            _ => panic!(),
        }
    }

    pub fn value(&self, value: Value) -> String {
        if let Some(constant) = self.function_data.value_to_constant.get(&value) {
            let data = self.function_data.constants().get(*constant);
            match data.value {
                ConstantValue::Integer { ty, value } => {
                    format!("{value}_{}", self.ty(ty))
                }
            }
        } else {
            format!("{value}")
        }
    }
}

pub(crate) fn format_instruction(
    formatter: &IrFormatter,
    instruction: &Instruction,
) -> String {
    match instruction {
        Instruction::ArithmeticBinary { dst, lhs, op, rhs } => {
            format!(
                "let {}: {} = {}.{} {}, {}",
                dst,
                formatter.value_type(*dst),
                op,
                formatter.value_type(*lhs),
                formatter.value(*lhs),
                formatter.value(*rhs)
            )
        }
        Instruction::ArithmeticUnary { dst, op, value } => {
            format!(
                "let {}: {} = {}.{} {}",
                dst,
                formatter.value_type(*dst),
                op,
                formatter.value_type(*value),
                value
            )
        }
        Instruction::Branch { target } => format!("branch {target}"),
        Instruction::BranchConditional {
            condition,
            on_true,
            on_false,
        } => format!("branch_if {condition} {on_true}, {on_false}"),
        Instruction::Call {
            function,
            arguments,
            dst,
        } => todo!(),
        Instruction::Cast {
            cast_op,
            to_type,
            dst,
            value,
        } => todo!(),
        Instruction::GetElementPtr { dst, ptr, index } => todo!(),
        Instruction::IntCompare {
            pred,
            dst,
            lhs,
            rhs,
        } => {
            format!(
                "let {}: {} = {}.{} {}, {}",
                dst,
                formatter.value_type(*dst),
                pred,
                formatter.value_type(*lhs),
                lhs,
                rhs
            )
        }
        Instruction::Load { dst, ptr } => {
            format!(
                "let {}: {} = load.{} {}",
                dst,
                formatter.value_type(*dst),
                formatter.value_type(*ptr),
                ptr
            )
        }
        Instruction::Return { value } => {
            if let Some(value) = value {
                format!("ret {}", value)
            } else {
                format!("ret")
            }
        }
        Instruction::Select {
            dst,
            condition,
            on_true,
            on_false,
        } => {
            format!(
                "let {}: {} = select {}, {}, {}",
                dst,
                formatter.value_type(*dst),
                condition,
                on_true,
                on_false
            )
        }
        Instruction::StackAlloc { dst, ty, size } => {
            format!(
                "let {}: {} = stack_alloc.{} {}",
                dst,
                formatter.value_type(*dst),
                formatter.ty(*ty),
                size
            )
        }
        Instruction::Store { ptr, value } => {
            format!("store.{} {}, {}", formatter.value_type(*ptr), ptr, value)
        }
        Instruction::Nop => format!("nop"),
    }
}
