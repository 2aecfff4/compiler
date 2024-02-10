//! A catalogue of optimizing transformations - Frances E. Allen, John Cocke
//! https://www.clear.rice.edu/comp512/Lectures/Papers/1971-allen-catalog.pdf

pub mod dead_code_elimination;
use crate::{context::Context, function::FunctionData, ty::Types};

///
pub(crate) struct FunctionContext<'a> {
    pub types: &'a Types,
    pub function: &'a mut FunctionData,
}

impl<'a> FunctionContext<'a> {
    pub fn new(types: &'a Types, function: &'a mut FunctionData) -> Self {
        Self { types, function }
    }
}

///
pub(crate) trait Pass {
    ///
    fn run(&mut self, ctx: &mut FunctionContext<'_>);
}
