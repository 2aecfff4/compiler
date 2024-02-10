use crate::{
    instruction::Instruction,
    passes::{FunctionContext, Pass},
};

///
#[derive(Default)]
pub(crate) struct SimplifyCfgPass;

impl Pass for SimplifyCfgPass {
    ///
    fn run(&mut self, ctx: &mut FunctionContext<'_>) {
        'merge_loop: loop {
            // #NOTE: Skip the first label, aka function entry point
            for target_label in ctx.function.labels().labels().skip(1) {
                let cfg = ctx.function.labels().cfg();
                let mut predecessors = cfg.incoming(target_label);

                // We only can merge labels when there is only a one incoming edge.
                let predecessors_count = predecessors.clone().count();
                if predecessors_count != 1 {
                    continue;
                }

                // https://en.wikipedia.org/wiki/Dominator_(graph_theory)
                let (from, to, _) = predecessors.next().unwrap();
                println!("[{target_label} -> [{from} -> {to}]");
                if from == target_label {
                    // It is a loop, skip
                    continue;
                }

                // Check the last instruction
                if !matches!(
                    ctx.function.last_instruction(from),
                    Instruction::Branch { .. }
                ) {
                    // We can only merge jumps
                    continue;
                }

                // Merge time
                let mut instructions = ctx.function.labels_mut().remove(target_label);
                let dominator_instructions =
                    &mut ctx.function.labels_mut().get_mut(from).instructions;

                // Remove jump
                dominator_instructions.pop();

                dominator_instructions.append(&mut instructions);

                continue 'merge_loop;
            }
            break;
        }
    }
}
