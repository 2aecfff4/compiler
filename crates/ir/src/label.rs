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
    pub fn entry(&self) -> Label {
        Label(0)
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
        assert_ne!(label, self.entry());

        let data = self.labels.remove(&label).unwrap();
        data.instructions
    }

    ///
    pub fn iter(&self) -> impl Iterator<Item = (&Label, &LabelData)> {
        self.labels.iter()
    }

    ///
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Label, &mut LabelData)> {
        self.labels.iter_mut()
    }

    ///
    pub fn labels(&self) -> impl Iterator<Item = &Label> {
        self.labels.keys()
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
