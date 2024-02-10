use crate::label::Label;

///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct Location {
    pub label: Label,
    pub instruction: u32,
}
