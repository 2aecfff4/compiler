use crate::{
    context::{function::Function, types::Types, values::Values},
    handles::{FunctionHandle, LabelHandle, TypeHandle, ValueHandle},
    instruction::{BinaryOp, CastOp, Instruction, IntCompareOp, UnaryOp},
};

pub(super) struct Label {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

//////////////////////////////////////////////////////////////////////////////////////////
// Labels

///
pub(super) struct Labels {
    labels: Vec<Label>,
}

impl Labels {
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }

    pub fn create(&mut self, name: &str) -> LabelHandle {
        let index = self.labels.len();
        self.labels.push(Label {
            name: name.to_string(),
            instructions: Vec::new(),
        });

        LabelHandle::new(index.try_into().unwrap())
    }

    pub fn get(&self, handle: LabelHandle) -> Option<&Label> {
        let index = handle.id();
        if index < self.labels.len() {
            Some(&self.labels[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, handle: LabelHandle) -> Option<&mut Label> {
        let index = handle.id();
        if index < self.labels.len() {
            Some(&mut self.labels[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &Label)> {
        self.labels
            .iter()
            .enumerate()
            .map(|(id, label)| (id as u32, label))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// LabelContext

macro_rules! impl_arithmetic_binary {
    {$(
        $(#[$($attrs:tt)*])*
        impl $name: ident for $op: expr
    ),*} => {
        $(
            $(#[$($attrs)*])*
            pub fn $name(&mut self, lhs: ValueHandle, rhs: ValueHandle) -> ValueHandle {
                self.arithmetic_binary(lhs, $op, rhs)
            }
        )*
    };
}

macro_rules! impl_arithmetic_unary {
    {$(
        $(#[$($attrs:tt)*])*
        impl $name: ident for $op: expr
    ),*} => {
        $(
            $(#[$($attrs)*])*
            pub fn $name(&mut self, value: ValueHandle) -> ValueHandle {
                self.arithmetic_unary($op, value)
            }
        )*
    };
}

macro_rules! impl_cast {
    {$(
        $(#[$($attrs:tt)*])*
        impl $name: ident for $op: expr
    ),*} => {
        $(
            $(#[$($attrs)*])*
            pub fn $name(&mut self, to_type: TypeHandle, value: ValueHandle) -> ValueHandle {
                self.cast($op, to_type, value)
            }
        )*
    };
}

macro_rules! impl_int_compare {
    {$(
        $(#[$($attrs:tt)*])*
        impl $name: ident for $op: expr
    ),*} => {
        $(
            $(#[$($attrs)*])*
            pub fn $name(&mut self, lhs: ValueHandle, rhs: ValueHandle,) -> ValueHandle {
                self.int_compare(lhs, $op, rhs)
            }
        )*
    };
}

///
pub struct LabelContext<'a> {
    types: &'a Types,
    values: &'a mut Values,
    label: &'a mut Label,
}

impl<'a> LabelContext<'a> {
    ///
    pub(super) fn new(
        types: &'a Types,
        values: &'a mut Values,
        label: &'a mut Label,
    ) -> Self {
        Self {
            types,
            values,
            label,
        }
    }

    impl_arithmetic_binary! {
        ///
        impl add for BinaryOp::Add,
        ///
        impl sub for BinaryOp::Sub,
        ///
        impl mul for BinaryOp::Mul,
        ///
        impl mod_ for BinaryOp::Mod,
        ///
        impl div for BinaryOp::Div,
        ///
        impl shr for BinaryOp::Shr,
        ///
        impl shl for BinaryOp::Shl,
        ///
        impl sar for BinaryOp::Sar,
        ///
        impl and for BinaryOp::And,
        ///
        impl or for BinaryOp::Or,
        ///
        impl xor for BinaryOp::Xor
    }

    impl_arithmetic_unary! {
        ///
        impl neg for UnaryOp::Neg,
        ///
        impl not for UnaryOp::Not
    }

    impl_cast! {
        ///
        impl bit_cast for CastOp::BitCast,
        ///
        impl sign_extend for CastOp::SignExtend,
        ///
        impl truncate for CastOp::Truncate,
        ///
        impl zero_extend for CastOp::ZeroExtend
    }

    impl_int_compare! {
       ///
       impl compare_eq for IntCompareOp::Equal,
       ///
       impl compare_ne for IntCompareOp::NotEqual,

       ///
       impl compare_gt for IntCompareOp::GreaterThan,
       ///
       impl compare_gte for IntCompareOp::GreaterThanOrEqual,

       ///
       impl compare_lt for IntCompareOp::LessThan,
       ///
       impl compare_lte for IntCompareOp::LessThanOrEqual
    }

    ///
    fn insert_instruction(&mut self, instruction: Instruction) {
        self.label.instructions.push(instruction);
    }

    ///
    fn with_output<Func>(&mut self, func: Func) -> ValueHandle
    where
        Func: FnOnce(ValueHandle) -> Instruction,
    {
        let value = self.values.alloc();
        let instruction = func(value);
        self.insert_instruction(instruction);

        value
    }

    ///
    fn arithmetic_binary(
        &mut self,
        lhs: ValueHandle, //
        op: BinaryOp,
        rhs: ValueHandle,
    ) -> ValueHandle {
        self.with_output(|dst| Instruction::ArithmeticBinary { dst, lhs, op, rhs })
    }

    ///
    fn arithmetic_unary(&mut self, op: UnaryOp, value: ValueHandle) -> ValueHandle {
        self.with_output(|dst| Instruction::ArithmeticUnary { dst, op, value })
    }

    ///
    pub fn branch(&mut self, target: LabelHandle) {
        self.insert_instruction(Instruction::Branch { target });
    }

    ///
    pub fn branch_conditional(
        &mut self,
        condition: ValueHandle,
        on_true: LabelHandle,
        on_false: LabelHandle,
    ) {
        self.insert_instruction(Instruction::BranchConditional {
            condition,
            on_true,
            on_false,
        });
    }

    /// #TODO:
    pub fn call(
        &mut self,
        function: FunctionHandle, //
        arguments: &[ValueHandle],
    ) -> Option<Vec<ValueHandle>> {
        todo!()
    }

    ///
    fn cast(
        &mut self,
        cast_op: CastOp,
        to_type: TypeHandle,
        value: ValueHandle,
    ) -> ValueHandle {
        self.with_output(|dst| Instruction::Cast {
            cast_op,
            to_type,
            dst,
            value,
        })
    }

    ///
    pub fn get_element_ptr(
        &mut self,
        source: ValueHandle, //
        index: ValueHandle,
    ) -> ValueHandle {
        self.with_output(|dst| Instruction::GetElementPtr { dst, source, index })
    }

    ///
    fn int_compare(
        &mut self,
        lhs: ValueHandle, //
        pred: IntCompareOp,
        rhs: ValueHandle,
    ) -> ValueHandle {
        self.with_output(|dst| Instruction::IntCompare {
            dst,
            lhs,
            pred,
            rhs,
        })
    }

    ///
    pub fn load(&mut self, ptr: ValueHandle) -> ValueHandle {
        self.with_output(|dst| Instruction::Load { dst, ptr })
    }

    ///
    pub fn ret(&mut self, values: Option<&[ValueHandle]>) {
        let values = values.map(|values| values.to_vec());
        self.insert_instruction(Instruction::Return { values });
    }

    ///
    pub fn select(
        &mut self,
        condition: ValueHandle,
        on_true: ValueHandle,
        on_false: ValueHandle,
    ) -> ValueHandle {
        self.with_output(|dst| Instruction::Select {
            dst,
            condition,
            on_true,
            on_false,
        })
    }

    ///
    pub fn stack_alloc(&mut self, ty: TypeHandle, size: usize) -> ValueHandle {
        self.with_output(|dst| Instruction::StackAlloc { dst, ty, size })
    }

    ///
    pub fn store(&mut self, ptr: ValueHandle, value: ValueHandle) {
        self.insert_instruction(Instruction::Store { ptr, value });
    }
}
