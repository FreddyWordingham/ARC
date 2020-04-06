//! Value reporting macro.

/// Report a list of values in evenly spaced columns.
#[macro_export]
macro_rules! values {
    ($col_width: expr, $($val: expr),*) => {
        let val_width = ($col_width / 4) - 1;
        let name_width = ($col_width / 2) - val_width - 2;
        crate::columns!($col_width,
            $(
                format!("{:>nw$} : {:<vw$}", stringify!($val), $val, nw = name_width, vw = val_width)
            ),*
        );
    };
}
