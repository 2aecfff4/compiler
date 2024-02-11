use std::collections::HashMap;

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
        self.optimize_jumps(ctx);
        self.optimize_branches(ctx);
    }
}

impl SimplifyCfgPass {
    ///
    fn optimize_jumps(&mut self, ctx: &mut FunctionContext<'_>) {
        let mut labels = Vec::new();
        'merge_loop: loop {
            // #TODO: Do something with it in the future.
            labels.clear();
            labels.extend(ctx.function.labels().labels());

            for &target_label in labels.iter() {
                if target_label == ctx.function.labels().entry() {
                    continue;
                }

                let cfg = ctx.function.labels().cfg();
                let mut predecessors = cfg.incoming(target_label);

                // We only can merge labels when there is only a one incoming edge.
                let predecessors_count = predecessors.clone().count();
                if predecessors_count != 1 {
                    continue;
                }

                // https://en.wikipedia.org/wiki/Dominator_(graph_theory)
                let (from, _, _) = predecessors.next().unwrap();
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

    ///
    fn optimize_branches(&mut self, ctx: &mut FunctionContext<'_>) {
        // Now optimize branches
        // For example:
        // block_0:
        //     branch_if v0 block_1 block_2
        //
        // block_2:
        //     branch block_3
        //
        // block_3:
        //     ...
        //
        // In this case `block_2` can be optimized away:
        // block_0:
        //     branch_if v0 block_1 block_3
        //
        // block_3:
        //     ...
        let mut branch_to_label = HashMap::new();
        for (label, data) in ctx.function.labels().iter() {
            let instructions = &data.instructions;
            if instructions.len() != 1 {
                continue;
            }

            if let Instruction::Branch { target } = ctx.function.last_instruction(*label)
            {
                if label == target {
                    // It is a loop, skip
                    continue;
                }

                branch_to_label.insert(*label, *target);
            }
        }

        let labels = ctx.function.labels().labels().cloned().collect::<Vec<_>>();

        for label in labels.iter() {
            if branch_to_label.get(label).is_some() {
                // It is a loop, skip
                continue;
            }

            match ctx.function.last_instruction_mut(*label) {
                Instruction::Branch { target } => {
                    if let Some(&new_target) = branch_to_label.get(target) {
                        *target = new_target;
                    }
                }
                Instruction::BranchConditional {
                    on_true, on_false, ..
                } => {
                    if let Some(&new_target) = branch_to_label.get(on_true) {
                        *on_true = new_target;
                    }
                    if let Some(&new_target) = branch_to_label.get(on_false) {
                        *on_false = new_target;
                    }
                }
                _ => {}
            }
        }
    }
}
