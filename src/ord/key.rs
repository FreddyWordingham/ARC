//! Key alias.

use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Construct a new key type.
macro_rules! key {
    ($key:ident) => {
        #[json]
        #[derive(Clone, PartialOrd, Eq, PartialEq, Ord)]
        pub struct $key(String);

        impl $key {
            #[inline]
            #[must_use]
            pub fn new(string: &str) -> Self {
                Self {
                    0: string.to_string(),
                }
            }

            #[inline]
            #[must_use]
            pub fn str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $key {
            #[inline]
            fn fmt(&self, fmt: &mut Formatter) -> Result {
                write!(fmt, "{}", self.0)
            }
        }
    };
}

key!(InterKey);
key!(MatKey);
key!(MeshKey);
key!(ReactKey);
key!(SpecKey);
key!(SurfKey);
