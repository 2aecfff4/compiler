use crate::{
    constant::ConstantValue,
    function::FunctionData,
    instruction::{BinaryOp, Instruction, IntCompareOp, UnaryOp},
    label::Label,
    location::Location,
    passes::{FunctionContext, Pass},
    ty::{Type, TypeKind, Types},
    value::Value,
};

enum Replacement {
    Constant {
        value: Value,
        constant: ConstantValue,
        location: Location,
        instruction_replacement: Option<Instruction>,
    },
    Instruction {
        location: Location,
        instruction: Instruction,
    },
}

///
#[derive(Default)]
pub(crate) struct ConstantFoldingPass;

impl Pass for ConstantFoldingPass {
    ///
    // #TODO: This is very ugly, ngl
    //
    fn run(&mut self, ctx: &mut FunctionContext<'_>) {
        fn arithmetic_binary(
            types: &Types,
            function: &FunctionData,
            location: Location,
            dst: &Value,
            lhs: &Value,
            rhs: &Value,
            op: &BinaryOp,
        ) -> Option<Replacement> {
            macro_rules! is_const {
                ($val:expr) => {
                    function.value_to_constant.contains_key($val)
                };
            }

            macro_rules! get_const {
                ($val:expr) => {{
                    let constant = function.value_to_constant.get($val).unwrap();
                    function.constants().get(*constant).value
                }};
            }

            macro_rules! propagate_binary {
                ($is_signed:expr, $op:expr, $lhs:expr, $rhs:expr, $unsigned: ty, $signed: ty) => {{
                    let result = match $op {
                        BinaryOp::Add => {
                            ($lhs as $unsigned).wrapping_add($rhs as $unsigned)
                        }
                        BinaryOp::Sub => {
                            ($lhs as $unsigned).wrapping_sub($rhs as $unsigned)
                        }
                        BinaryOp::Mul => {
                            ($lhs as $unsigned).wrapping_mul($rhs as $unsigned)
                        }
                        BinaryOp::Mod => {
                            if $is_signed {
                                (($lhs as $signed) % ($rhs as $signed)) as $unsigned
                            } else {
                                (($lhs as $unsigned) % ($rhs as $unsigned))
                            }
                        }
                        BinaryOp::Div => {
                            if $is_signed {
                                (($lhs as $signed) / ($rhs as $signed)) as $unsigned
                            } else {
                                ($lhs as $unsigned) / ($rhs as $unsigned)
                            }
                        }
                        BinaryOp::Shr => ($lhs as $unsigned) >> ($rhs as $unsigned),
                        BinaryOp::Shl => ($lhs as $unsigned) << ($rhs as $unsigned),
                        BinaryOp::Sar => (($lhs as $signed) >> $rhs) as $unsigned,
                        BinaryOp::And => ($lhs as $unsigned) & ($rhs as $unsigned),
                        BinaryOp::Or => ($lhs as $unsigned) | ($rhs as $unsigned),
                        BinaryOp::Xor => ($lhs as $unsigned) ^ ($rhs as $unsigned),
                        BinaryOp::BitAnd => ($lhs as $unsigned) & ($rhs as $unsigned),
                        BinaryOp::BitOr => ($lhs as $unsigned) | ($rhs as $unsigned),
                    };
                    result as u64
                }};
            }

            if !is_const!(lhs) || !is_const!(rhs) {
                return None;
            }
            let lhs_const = get_const!(lhs);
            let rhs_const = get_const!(rhs);
            if !types.types_match(lhs_const.ty(), rhs_const.ty()) {
                return None;
            }

            let ty = lhs_const.ty();
            let type_kind = types.get(ty).type_kind();
            match type_kind {
                TypeKind::Integer {
                    num_bits,
                    is_signed,
                } => {
                    let lhs = lhs_const.integer().unwrap();
                    let rhs = rhs_const.integer().unwrap();

                    let value = match num_bits {
                        1 => propagate_binary!(*is_signed, op, lhs, rhs, u8, i8),
                        8 => propagate_binary!(*is_signed, op, lhs, rhs, u8, i8),
                        16 => propagate_binary!(*is_signed, op, lhs, rhs, u16, i16),
                        32 => propagate_binary!(*is_signed, op, lhs, rhs, u32, i32),
                        64 => propagate_binary!(*is_signed, op, lhs, rhs, u64, i64),
                        _ => panic!(),
                    };

                    Some(Replacement::Constant {
                        value: *dst,
                        constant: ConstantValue::Integer { ty, value },
                        location,
                        instruction_replacement: Some(Instruction::Nop),
                    })
                }
                _ => todo!(),
            }
        }

        ///
        fn arithmetic_unary(
            types: &Types,
            function: &FunctionData,
            location: Location,
            dst: &Value,
            value: &Value,
            op: &UnaryOp,
        ) -> Option<Replacement> {
            macro_rules! is_const {
                ($val:expr) => {
                    function.value_to_constant.contains_key($val)
                };
            }

            macro_rules! get_const {
                ($val:expr) => {{
                    let constant = function.value_to_constant.get($val).unwrap();
                    function.constants().get(*constant).value
                }};
            }

            macro_rules! propagate_unary {
                ($is_signed:expr, $op:expr, $value:expr, $unsigned: ty) => {{
                    let result = match $op {
                        UnaryOp::Neg => ($value as $unsigned).wrapping_neg(),
                        UnaryOp::Not => !($value as $unsigned),
                    };
                    result as u64
                }};
            }

            if !is_const!(value) {
                return None;
            }
            let value_const = get_const!(value);

            let ty = value_const.ty();
            let type_kind = types.get(ty).type_kind();
            match type_kind {
                TypeKind::Integer { num_bits, .. } => {
                    let value = value_const.integer().unwrap();

                    let value = match num_bits {
                        1 => propagate_unary!(*is_signed, op, value, u8),
                        8 => propagate_unary!(*is_signed, op, value, u8),
                        16 => propagate_unary!(*is_signed, op, value, u16),
                        32 => propagate_unary!(*is_signed, op, value, u32),
                        64 => propagate_unary!(*is_signed, op, value, u64),
                        _ => panic!(),
                    };

                    Some(Replacement::Constant {
                        value: *dst,
                        constant: ConstantValue::Integer { ty, value },
                        location,
                        instruction_replacement: Some(Instruction::Nop),
                    })
                }
                _ => todo!(),
            }
        }

        ///
        fn branch_conditional(
            function: &FunctionData,
            location: Location,
            condition: &Value,
            on_true: &Label,
            on_false: &Label,
        ) -> Option<Replacement> {
            macro_rules! is_const {
                ($val:expr) => {
                    function.value_to_constant.contains_key($val)
                };
            }

            macro_rules! get_const {
                ($val:expr) => {{
                    let constant = function.value_to_constant.get($val).unwrap();
                    function.constants().get(*constant).value
                }};
            }

            if !is_const!(condition) {
                return None;
            }
            let condition_const = get_const!(condition);
            let integer = condition_const.integer().unwrap();

            match integer {
                0 => Some(Replacement::Instruction {
                    location,
                    instruction: Instruction::Branch { target: *on_false },
                }),
                1 => Some(Replacement::Instruction {
                    location,
                    instruction: Instruction::Branch { target: *on_true },
                }),
                _ => panic!(),
            }
        }

        ///
        fn int_compare(
            types: &Types,
            function: &FunctionData,
            location: Location,
            pred: &IntCompareOp,
            dst: &Value,
            lhs: &Value,
            rhs: &Value,
        ) -> Option<Replacement> {
            macro_rules! is_const {
                ($val:expr) => {
                    function.value_to_constant.contains_key($val)
                };
            }

            macro_rules! get_const {
                ($val:expr) => {{
                    let constant = function.value_to_constant.get($val).unwrap();
                    function.constants().get(*constant).value
                }};
            }

            macro_rules! propagate_compare {
                ($is_signed:expr, $op:expr, $lhs:expr, $rhs:expr, $unsigned: ty, $signed: ty) => {{
                    let result = match $op {
                        IntCompareOp::Equal => {
                            if $is_signed {
                                ($lhs as $signed) == ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) == ($rhs as $unsigned)
                            }
                        }
                        IntCompareOp::NotEqual => {
                            if $is_signed {
                                ($lhs as $signed) != ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) != ($rhs as $unsigned)
                            }
                        }
                        IntCompareOp::GreaterThan => {
                            if $is_signed {
                                ($lhs as $signed) > ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) > ($rhs as $unsigned)
                            }
                        }
                        IntCompareOp::GreaterThanOrEqual => {
                            if $is_signed {
                                ($lhs as $signed) >= ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) >= ($rhs as $unsigned)
                            }
                        }
                        IntCompareOp::LessThan => {
                            if $is_signed {
                                ($lhs as $signed) < ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) < ($rhs as $unsigned)
                            }
                        }
                        IntCompareOp::LessThanOrEqual => {
                            if $is_signed {
                                ($lhs as $signed) <= ($rhs as $signed)
                            } else {
                                ($lhs as $unsigned) <= ($rhs as $unsigned)
                            }
                        }
                    };
                    result as u64
                }};
            }

            if !is_const!(lhs) || !is_const!(rhs) {
                return None;
            }
            let lhs_const = get_const!(lhs);
            let rhs_const = get_const!(rhs);
            if !types.types_match(lhs_const.ty(), rhs_const.ty()) {
                return None;
            }

            let ty = lhs_const.ty();
            let type_kind = types.get(ty).type_kind();
            match type_kind {
                TypeKind::Integer {
                    num_bits,
                    is_signed,
                } => {
                    let lhs = lhs_const.integer().unwrap();
                    let rhs = rhs_const.integer().unwrap();

                    let value = match num_bits {
                        1 => propagate_compare!(*is_signed, pred, lhs, rhs, u8, i8),
                        8 => propagate_compare!(*is_signed, pred, lhs, rhs, u8, i8),
                        16 => propagate_compare!(*is_signed, pred, lhs, rhs, u16, i16),
                        32 => propagate_compare!(*is_signed, pred, lhs, rhs, u32, i32),
                        64 => propagate_compare!(*is_signed, pred, lhs, rhs, u64, i64),
                        _ => panic!(),
                    };

                    Some(Replacement::Constant {
                        value: *dst,
                        constant: ConstantValue::Integer { ty, value },
                        location,
                        instruction_replacement: Some(Instruction::Nop),
                    })
                }
                _ => todo!(),
            }
        }

        ///
        fn select(
            types: &Types,
            function: &FunctionData,
            location: Location,
            dst: &Value,
            condition: &Value,
            on_true: &Value,
            on_false: &Value,
        ) -> Option<Replacement> {
            macro_rules! is_const {
                ($val:expr) => {
                    function.value_to_constant.contains_key($val)
                };
            }

            macro_rules! get_const {
                ($val:expr) => {{
                    let constant = function.value_to_constant.get($val).unwrap();
                    function.constants().get(*constant).value
                }};
            }

            if !is_const!(condition) {
                return None;
            }
            let condition_const = get_const!(condition);
            let integer = condition_const.integer().unwrap();

            todo!()

            // match integer {
            //     0 => Some(Replacement::Instruction {
            //         location,
            //         instruction: Instruction::Branch { target: *on_false },
            //     }),
            //     1 => Some(Replacement::Instruction {
            //         location,
            //         instruction: Instruction::Branch { target: *on_true },
            //     }),
            //     _ => panic!(),
            // }
        }

        // Try to propagate constants iteratively.
        loop {
            let cfg = ctx.function.labels().cfg();
            let mut replacements = Vec::new();

            cfg.bfs(|label| {
                for (idx, instr) in ctx
                    .function
                    .labels()
                    .get(label)
                    .instructions
                    .iter()
                    .enumerate()
                {
                    let location = Location {
                        label,
                        instruction: idx as u32,
                    };
                    let replacement = match instr {
                        Instruction::ArithmeticBinary { dst, lhs, op, rhs } => {
                            arithmetic_binary(
                                ctx.types,
                                ctx.function,
                                location,
                                dst,
                                lhs,
                                rhs,
                                op,
                            )
                        }
                        Instruction::ArithmeticUnary { dst, op, value } => {
                            arithmetic_unary(
                                ctx.types,
                                ctx.function,
                                location,
                                dst,
                                value,
                                op,
                            )
                        }
                        Instruction::BranchConditional {
                            condition,
                            on_true,
                            on_false,
                        } => branch_conditional(
                            ctx.function,
                            location,
                            condition,
                            on_true,
                            on_false,
                        ),
                        Instruction::Cast {
                            cast_op,
                            to_type,
                            dst,
                            value,
                        } => None,
                        Instruction::IntCompare {
                            pred,
                            dst,
                            lhs,
                            rhs,
                        } => int_compare(
                            ctx.types,
                            ctx.function,
                            location,
                            pred,
                            dst,
                            lhs,
                            rhs,
                        ),
                        Instruction::Select {
                            dst,
                            condition,
                            on_true,
                            on_false,
                        } => None,
                        _ => None,
                    };

                    if let Some(replacement) = replacement {
                        replacements.push(replacement);
                    }
                }
            });

            if replacements.is_empty() {
                break;
            }

            for replacement in replacements {
                // Swap all users
                let users = ctx.function.variable_users();
                replace(replacement, ctx, users);
            }
        }
    }
}

fn replace(
    replacement: Replacement,
    ctx: &mut FunctionContext<'_>,
    users: std::collections::HashMap<Value, std::collections::HashSet<Location>>,
) {
    match replacement {
        Replacement::Constant {
            value: needle,
            constant,
            location,
            instruction_replacement,
        } => {
            let constant = ctx.function.alloc_constant(constant);

            if let Some(replacement) = instruction_replacement {
                *ctx.function.instruction_mut(&location) = replacement;
            }

            let variable_users = users.get(&needle).unwrap();
            for user in variable_users.iter() {
                match ctx.function.instruction_mut(user) {
                    Instruction::ArithmeticBinary { dst, lhs, rhs, .. } => {
                        let values = [dst, lhs, rhs];
                        for value in values {
                            if *value == needle {
                                *value = constant;
                            }
                        }
                    }
                    Instruction::ArithmeticUnary { value, .. } => {
                        if *value == needle {
                            *value = constant;
                        }
                    }
                    Instruction::Call { arguments, .. } => {
                        for arg in arguments.iter_mut() {
                            if *arg == needle {
                                *arg = constant;
                            }
                        }
                    }
                    Instruction::Cast {
                        cast_op,
                        to_type,
                        dst,
                        value,
                    } => todo!(),
                    Instruction::GetElementPtr { dst, ptr, index } => {}
                    Instruction::IntCompare { dst, lhs, rhs, .. } => {
                        let values = [dst, lhs, rhs];
                        for value in values {
                            if *value == needle {
                                *value = constant;
                            }
                        }
                    }
                    Instruction::Load { ptr, .. } => {
                        if *ptr == needle {
                            *ptr = constant;
                        }
                    }
                    Instruction::Return { value: Some(value) } => {
                        if *value == needle {
                            *value = constant;
                        }
                    }
                    Instruction::Select {
                        dst,
                        condition,
                        on_true,
                        on_false,
                    } => {
                        let values = [dst, condition, on_true, on_false];
                        for value in values {
                            if *value == needle {
                                *value = constant;
                            }
                        }
                    }
                    Instruction::Store { value, .. } => {
                        if *value == needle {
                            *value = constant;
                        }
                    }
                    _ => {}
                }
            }
        }
        Replacement::Instruction {
            location,
            instruction,
        } => {
            *ctx.function.instruction_mut(&location) = instruction;
        }
    }
}
