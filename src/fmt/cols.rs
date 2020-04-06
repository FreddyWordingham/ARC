//! Column printing.

/// Print to equally spaced columns.
#[macro_export]
macro_rules! cols {
    ($num_cols: expr, $total_width: expr, $($val: expr),*) => {
        {
            use std::fmt::Write;

            let col_width = $total_width / $num_cols;

            let mut string = String::new();
            $(write!(string, "{:<width$}", $val, width = col_width).expect("Failed to write to string buffer.");)*

            string
        }
    };
}
