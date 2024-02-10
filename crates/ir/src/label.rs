use crate::{handle_impl, instruction::Instruction};
use smallvec::{smallvec, SmallVec};
use std::collections::{HashMap, HashSet, VecDeque};

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
    labels: HashMap<Label, LabelData>,
    current_id: u32,
}

impl Labels {
    ///
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            current_id: 0,
        }
    }

    ///
    pub fn create(&mut self, name: &str) -> Label {
        let id = self.current_id;
        self.current_id += 1;

        self.labels.insert(
            Label(id),
            LabelData {
                name: name.to_string(),
                instructions: Vec::new(),
            },
        );

        Label(id)
    }

    ///
    pub fn get(&self, label: Label) -> &LabelData {
        self.labels.get(&label).unwrap()
    }

    ///
    pub fn get_mut(&mut self, label: Label) -> &mut LabelData {
        self.labels.get_mut(&label).unwrap()
    }

    ///
    pub fn remove(&mut self, label: Label) -> Vec<Instruction> {
        let data = self.labels.remove(&label).unwrap();
        data.instructions
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (&Label, &LabelData)> {
        self.labels.iter()
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Label, &mut LabelData)> {
        self.labels.iter_mut()
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
    pub fn labels(&self) -> impl Iterator<Item = Label> {
        let count = self.labels.len() as u32;
        (0..count).map(Label)
    }

    /// Retrieves the target labels associated with the last instruction of a specific label.
    /// This method assumes that the label exists in the instruction map.
    ///
    /// Returns a [`SmallVec`] containing the target labels or panics if the label or targets are not present.
    pub(crate) fn targets(&self, label: Label) -> SmallVec<[Label; 8]> {
        if let Some(last) = self.get(label).instructions.last() {
            last.targets().unwrap_or_else(|| smallvec![])
        } else {
            smallvec![]
        }
    }

    /// #TODO: This probably should not be regenerated every single time.
    pub fn cfg(&self) -> crate::cfg::Cfg {
        crate::cfg::Cfg::new(self)
    }
}
