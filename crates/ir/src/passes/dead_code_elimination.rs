use std::collections::HashMap;

use crate::{
    instruction::Instruction,
    location::Location,
    passes::{FunctionContext, Pass},
    pointer_analysis::PointerAnalysis,
    value::Value,
};

///
#[derive(Default)]
pub(crate) struct DeadCodeEliminationPass;

impl Pass for DeadCodeEliminationPass {
    /// This pass tries to remove dead code. For example:
    /// ```
    /// fn @test(v0: u32, v1: u32)  {
    ///     block_0: {
    ///         let v2: *u32 = stack_alloc.u32 1
    ///         store.*u32 v2, v0
    ///         let v3: *u32 = stack_alloc.u32 1
    ///         store.*u32 v3, v1
    ///         // \/ this add instruction will be removed
    ///         let v4: u32 = add.u32 v0, v1
    ///         ret
    ///     }
    /// }
    /// ```
    fn run(&mut self, ctx: &mut FunctionContext<'_>) {
        let mut to_remove_queue = Vec::new();
        let pointer_analysis = ctx.function.pointer_analysis(ctx.types);
        let mut uses = ctx
            .function
            .variable_users()
            .iter()
            .map(|(value, locations)| (*value, locations.len() as i32))
            .collect::<HashMap<_, _>>();

        for (_, location) in pointer_analysis.creators.iter() {
            Self::try_to_remove(
                ctx,
                location,
                &mut uses,
                &pointer_analysis,
                &mut to_remove_queue,
            );
        }

        while let Some(location) = to_remove_queue.pop() {
            Self::try_to_remove(
                ctx,
                &location,
                &mut uses,
                &pointer_analysis,
                &mut to_remove_queue,
            );
        }
    }
}

impl DeadCodeEliminationPass {
    ///
    pub fn can_be_removed(instr: &Instruction) -> bool {
        !matches!(
            instr,
            Instruction::Branch { .. }
                | Instruction::BranchConditional { .. }
                | Instruction::Call { .. }
                | Instruction::Return { .. }
                | Instruction::Store { .. }
        )
    }

    fn try_to_remove(
        ctx: &mut FunctionContext<'_>,
        location: &Location,
        uses: &mut HashMap<Value, i32>,
        pointer_analysis: &PointerAnalysis,
        to_remove_queue: &mut Vec<Location>,
    ) {
        let instr = ctx.function.instruction_mut(location);
        if matches!(instr, crate::instruction::Instruction::Nop) {
            return;
        }
        let creates = instr.creates().unwrap();
        let uses_count = uses.get_mut(&creates).unwrap();

        if Self::can_be_removed(instr) && *uses_count == 0 {
            let reads = instr.reads();
            *instr = Instruction::Nop;

            if let Some(reads) = reads {
                for read in reads.iter() {
                    let uses = uses.get_mut(read).unwrap();
                    uses.checked_sub(1).unwrap();

                    if *uses == 0 {
                        if let Some(creator) = pointer_analysis.creators.get(read) {
                            to_remove_queue.push(*creator);
                        }
                    }
                }
            }
        }
    }
}
