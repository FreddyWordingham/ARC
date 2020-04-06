//! Column printing.

/// Print to equally spaced columns.
#[macro_export]
macro_rules! cols {
    ($num_cols: expr, $total_width: expr, $($val: expr),*) => {
        {
            use std::fmt::Write;

            let mut string = String::new();
            $(write!(string, "{:<width$}", $val, width = $total_width / $num_cols).expect("Failed to write to string buffer.");)*

            string
        }
    };
}
