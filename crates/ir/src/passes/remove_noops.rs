use crate::{
    instruction::Instruction,
    passes::{FunctionContext, Pass},
};

///
#[derive(Default)]
pub(crate) struct RemoveNoopsPass;

impl Pass for RemoveNoopsPass {
    ///
    fn run(&mut self, ctx: &mut FunctionContext<'_>) {
        for (_, data) in ctx.function.labels_mut().iter_mut() {
            data.instructions
                .retain(|instr| !matches!(instr, Instruction::Nop));
        }
    }
}
