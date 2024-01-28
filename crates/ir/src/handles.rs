///
#[derive(Debug, Clone, Copy, Hash)]
pub struct Handle<const TYPE: u32> {
    handle: u32,
}

impl<const TYPE: u32> Handle<TYPE> {
    pub(crate) fn new(handle: u32) -> Self {
        Self { handle }
    }

    pub(crate) fn id(&self) -> usize {
        self.handle as usize
    }
}

///
pub type TypeHandle = Handle<0>;

///
pub type ValueHandle = Handle<1>;

///
pub type LabelHandle = Handle<2>;

///
pub type FunctionHandle = Handle<3>;
