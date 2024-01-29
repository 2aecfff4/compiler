#[macro_export]
macro_rules! handle_impl {
    {
        $(#[$($attrs:tt)*])*
        impl $name: ident
    } => {
        $(#[$($attrs)*])*
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name(u32);

        impl $name {
            pub(crate) fn id(&self) -> usize {
                self.0 as usize
            }
        }
    };
}
