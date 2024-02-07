use crate::{handle_impl, instruction::Instruction};
use smallvec::SmallVec;
use std::collections::{HashSet, VecDeque};

handle_impl! {
    ///
    impl Label
}

pub(crate) struct LabelData {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

//////////////////////////////////////////////////////////////////////////////////////////
// Labels

///
pub(crate) struct Labels {
    labels: Vec<LabelData>,
}

impl Labels {
    ///
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }

    ///
    pub fn create(&mut self, name: &str) -> Label {
        let index = self.labels.len();
        self.labels.push(LabelData {
            name: name.to_string(),
            instructions: Vec::new(),
        });

        Label(index.try_into().unwrap())
    }

    ///
    pub fn get(&self, handle: Label) -> &LabelData {
        let index = handle.id();
        self.labels.get(index).unwrap()
    }

    ///
    pub fn get_mut(&mut self, handle: Label) -> &mut LabelData {
        let index = handle.id();
        self.labels.get_mut(index).unwrap()
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (Label, &LabelData)> {
        self.labels
            .iter()
            .enumerate()
            .map(|(id, label)| (Label(id as u32), label))
    }

    /// Retrieves the target labels associated with the last instruction of a specific label.
    /// This method assumes that the label exists in the instruction map.
    ///
    /// Returns a [`SmallVec`] containing the target labels or panics if the label or targets are not present.
    fn targets(&self, label: Label) -> SmallVec<[Label; 8]> {
        self.get(label)
            .instructions
            .last()
            .unwrap()
            .targets()
            .unwrap()
    }

    ///
    pub fn traverse_bfs(&self, start: Option<Label>, mut callback: impl FnMut(Label)) {
        let num_labels = self.labels.len();
        let mut visited = HashSet::with_capacity(num_labels);
        let mut queue = VecDeque::with_capacity(num_labels / 2);

        let start = start.unwrap_or_else(|| Label(0));
        queue.push_back(start);

        while let Some(label) = queue.pop_front() {
            if !visited.insert(label) {
                continue;
            }

            callback(label);
            for target in self.targets(label).iter() {
                queue.push_back(*target);
            }
        }

    /// #TODO: This probably should not be regenerated every single time.
    pub fn cfg(&self) -> crate::cfg::Cfg {
        crate::cfg::Cfg::new(self)
    }
}
