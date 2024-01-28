use crate::handles::ValueHandle;

///
pub(super) struct Values {
    current_id: u32,
}

impl Values {
    pub fn new() -> Self {
        Self { current_id: 0 }
    }

    pub fn alloc(&mut self) -> ValueHandle {
        let id = self.current_id;
        self.current_id += 1;

        ValueHandle::new(id)
    }
}
