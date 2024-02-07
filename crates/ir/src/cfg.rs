use crate::{
    instruction::Instruction,
    label::{Label, Labels},
};
use petgraph::graphmap::DiGraphMap;

///
#[derive(Debug, Clone, Copy)]
pub(crate) enum EdgeType {
    Jump,
    True,
    False,
}

///
#[derive(Debug, Clone)]
pub(crate) struct Cfg {
    graph: DiGraphMap<Label, EdgeType>,
}

impl Cfg {
    pub fn new(labels: &Labels) -> Self {
        let mut graph = DiGraphMap::new();

        for (label, _) in labels.iter() {
            graph.add_node(label);
        }

        for (label, data) in labels.iter() {
            for target in labels.targets(label) {
                let edge = match data.instructions.last().unwrap() {
                    Instruction::Branch { .. } => EdgeType::Jump,
                    Instruction::BranchConditional { on_true, .. } => {
                        if target == *on_true {
                            EdgeType::True
                        } else {
                            EdgeType::False
                        }
                    }
                    _ => panic!(),
                };
                graph.add_edge(label, target, edge);
            }
        }

        Self { graph }
    }

    ///
    pub fn bfs(&self, mut visitor: impl FnMut(Label)) {
        let mut bfs = petgraph::visit::Bfs::new(&self.graph, Label(0));
        while let Some(label) = bfs.next(&self.graph) {
            visitor(label);
        }
    }

    ///
    pub fn incoming(&self, label: Label) -> impl Iterator + '_ {
        self.graph
            .edges_directed(label, petgraph::Direction::Incoming)
    }

    ///
    pub fn outgoing(&self, label: Label) -> impl Iterator + '_ {
        self.graph
            .edges_directed(label, petgraph::Direction::Outgoing)
    }
}
