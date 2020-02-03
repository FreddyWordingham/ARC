//! Access macro.

/// Succinctly create a reference method for the given variable.
#[macro_export]
macro_rules! access {
    ($field:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub fn $field(&self) -> &$type {
            &self.$field
        }
    };

    ($field:ident, $setter:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub fn $field(&self) -> &$type {
            &self.$field
        }

        #[inline]
        #[must_use]
        pub fn $setter(&mut self) -> &mut $type {
            &mut self.$field
        }
    };
}
