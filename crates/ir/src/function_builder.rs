use crate::{
    constant::ConstantValue,
    function::{Function, FunctionData, Functions},
    instruction::{BinaryOp, CastOp, Instruction, IntCompareOp, UnaryOp},
    label::{Label, LabelData},
    ty::{Type, TypeKind, Types},
    value::{Value, Values},
};

macro_rules! impl_arithmetic_binary {
    {$(
        $(#[$($attrs:tt)*])*
        impl $name: ident for $op: expr
    ),*} => {
        $(
            $(#[$($attrs)*])*
            pub fn $name(&mut self, lhs: Value, rhs: Value) -> Value {
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
            pub fn $name(&mut self, value: Value) -> Value {
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
            pub fn $name(&mut self, to_type: Type, value: Value) -> Value {
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
            pub fn $name(&mut self, lhs: Value, rhs: Value,) -> Value {
                self.int_compare(lhs, $op, rhs)
            }
        )*
    };
}

///
pub struct FunctionBuilder<'a> {
    types: &'a mut Types,
    function: &'a mut FunctionData,
    current_label: Option<Label>,
}

impl<'a> FunctionBuilder<'a> {
    ///
    pub(crate) fn new(types: &'a mut Types, function: &'a mut FunctionData) -> Self {
        Self {
            types,
            function,
            current_label: None,
        }
    }

    ///
    pub fn create_type(&mut self, ty: TypeKind) -> Type {
        self.types.create(ty)
    }

    ///
    pub fn parameter(&self, index: usize) -> Value {
        self.function.parameters()[index]
    }

    ///
    pub fn create_label(&mut self, name: &str) -> Label {
        self.function.labels_mut().create(name)
    }

    ///
    pub fn alloc_constant(&mut self, value: ConstantValue) -> Value {
        let ty = value.ty();
        let constant = self.function.constants.create(value);
        let value = self.function.values_mut().alloc(ty);
        self.function.value_to_constant.insert(value, constant);

        value
    }

    ///
    pub fn set_insert_point(&mut self, label: Label) {
        self.current_label = Some(label);
    }

    fn label(&mut self) -> &mut LabelData {
        self.function
            .labels_mut()
            .get_mut(self.current_label.unwrap())
    }

    fn values(&mut self) -> &mut Values {
        self.function.values_mut()
    }

    ///
    fn insert_instruction(&mut self, instruction: Instruction) {
        self.label().instructions.push(instruction);
    }

    ///
    fn with_output<Func>(&mut self, ty: Type, func: Func) -> Value
    where
        Func: FnOnce(Value) -> Instruction,
    {
        let value = self.values().alloc(ty);
        let instruction = func(value);
        self.insert_instruction(instruction);

        value
    }

    fn validate_values_types(
        &mut self,
        a: Value, //
        b: Value,
    ) -> Type {
        let a_type = self.values().get(a).ty();
        let b_type = self.values().get(b).ty();

        assert!(self.types.types_match(a_type, b_type));
        a_type
    }

    ///
    fn arithmetic_binary(
        &mut self,
        lhs: Value, //
        op: BinaryOp,
        rhs: Value,
    ) -> Value {
        let ty = self.validate_values_types(lhs, rhs);

        self.with_output(ty, |dst| Instruction::ArithmeticBinary {
            dst,
            lhs,
            op,
            rhs,
        })
    }

    ///
    fn arithmetic_unary(&mut self, op: UnaryOp, value: Value) -> Value {
        let ty = self.values().get(value).ty();

        self.with_output(ty, |dst| Instruction::ArithmeticUnary { dst, op, value })
    }

    ///
    pub fn branch(&mut self, target: Label) {
        self.insert_instruction(Instruction::Branch { target });
    }

    ///
    pub fn branch_conditional(
        &mut self,
        condition: Value,
        on_true: Label,
        on_false: Label,
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
        function: Function, //
        arguments: &[Value],
    ) -> Option<Vec<Value>> {
        todo!()
    }

    ///
    fn cast(&mut self, cast_op: CastOp, to_type: Type, value: Value) -> Value {
        self.with_output(to_type, |dst| Instruction::Cast {
            cast_op,
            to_type,
            dst,
            value,
        })
    }

    ///
    pub fn get_element_ptr(
        &mut self,
        ptr: Value, //
        index: Value,
    ) -> Value {
        let ptr_type = self.values().get(ptr).ty();
        let index_type = self.values().get(index).ty();
        assert!(self.types.is_pointer(ptr_type));

        let ty = self.types.strip_pointer(ptr_type).unwrap();

        if self.types.is_struct(ty) {
            assert!(self.types.is_arithmetic(index_type));
            // #TODO: The `index` should be a constant
            todo!()
        } else if self.types.is_pointer(ty) {
            self.with_output(ptr_type, |dst| Instruction::GetElementPtr {
                dst,
                ptr,
                index,
            })
        } else {
            panic!("Invalid `ptr` type")
        }
    }

    ///
    fn int_compare(
        &mut self,
        lhs: Value, //
        pred: IntCompareOp,
        rhs: Value,
    ) -> Value {
        self.validate_values_types(lhs, rhs);

        // TODO: It is a basic type. It should not be created every single time.
        let ty = self.types.create(TypeKind::Integer {
            num_bits: 1,
            is_signed: false,
        });
        self.with_output(ty, |dst| Instruction::IntCompare {
            dst,
            lhs,
            pred,
            rhs,
        })
    }

    ///
    pub fn load(&mut self, ptr: Value) -> Value {
        let ptr_type = self.values().get(ptr).ty();
        assert!(self.types.is_pointer(ptr_type));

        let ty_handle = self.values().get(ptr).ty();
        let ty = self.types.strip_pointer(ty_handle).unwrap();

        self.with_output(ty, |dst| Instruction::Load { dst, ptr })
    }

    ///
    pub fn ret(&mut self, value: Option<Value>) {
        self.insert_instruction(Instruction::Return { value });
    }

    ///
    pub fn select(&mut self, condition: Value, on_true: Value, on_false: Value) -> Value {
        let ty = self.validate_values_types(on_true, on_false);
        // #TODO: Ensure that the condition is 1 bit wide.

        self.with_output(ty, |dst| Instruction::Select {
            dst,
            condition,
            on_true,
            on_false,
        })
    }

    ///
    pub fn stack_alloc(&mut self, ty: Type, size: usize) -> Value {
        let ret_ty = self.types.add_pointer(ty);
        self.with_output(ret_ty, |dst| Instruction::StackAlloc { dst, ty, size })
    }

    ///
    pub fn store(&mut self, ptr: Value, value: Value) {
        let ptr_type = self.values().get(ptr).ty();
        assert!(self.types.is_pointer(ptr_type));

        self.insert_instruction(Instruction::Store { ptr, value });
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
        impl xor for BinaryOp::Xor,
        ///
        impl bit_and for BinaryOp::BitAnd,
        ///
        impl bit_or for BinaryOp::BitOr
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
}
