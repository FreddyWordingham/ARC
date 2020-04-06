//! Value reporting macro.

use crate::fmt::columns;
use std::fmt::Display;

/// Width allocated to value name printing.
pub const NAME_WIDTH: usize = 32;

/// Report a list of values in evenly spaced columns.
#[macro_export]
macro_rules! values {
    ($col_width: expr, $($val: expr),*) => {
        debug_assert!(($col_width * 2) > crate::fmt::NAME_WIDTH);

        let name_width = crate::fmt::NAME_WIDTH;
        let val_width = $col_width - crate::fmt::NAME_WIDTH - 3;
        crate::columns!($col_width,
            $(
                format!("{:>nw$} : {:<vw$}", stringify!($val), $val, nw = name_width, vw = val_width)
            ),*
        );
    };
}

/// Print a given iterator of name-value pairs into equally spaced columns.
#[inline]
pub fn values<I, T, S>(col_width: usize, values: I)
where
    I: IntoIterator<Item = (T, S)>,
    T: Display,
    S: Display,
{
    let val_width = col_width - NAME_WIDTH - 3;
    columns(
        col_width,
        values.into_iter().map(|(name, value)| {
            format!(
                "{:>nw$} : {:<vw$}",
                name,
                value,
                nw = NAME_WIDTH,
                vw = val_width
            )
        }),
    );
}
