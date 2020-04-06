//! Value reporting macro.

/// Width allocated to value name printing.
pub const NAME_WIDTH: usize = 24;

/// Report a list of values in evenly spaced columns.
#[macro_export]
macro_rules! values {
    ($col_width: expr, $($val: expr),*) => {
        let name_width = (($col_width / 2) - 3).min(crate::fmt::NAME_WIDTH);
        let val_width = ($col_width / 2) - 1;
        crate::columns!($col_width,
            $(
                format!("{:>nw$} : {:<vw$}", stringify!($val), $val, nw = name_width, vw = val_width)
            ),*
        );
    };
}
