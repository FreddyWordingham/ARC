//! Column printing.

/// Print to equally spaced columns.
#[macro_export]
macro_rules! cols {
    ($num_cols: expr, $total_width: expr, $e0: expr) => {
        format!("{:<width$}", $e0, width = $total_width / $num_cols)
    };
    ($num_cols: expr, $total_width: expr, $e0: expr, $e1: expr) => {
        format!(
            "{:<width$}{:<width$}",
            $e0,
            $e1,
            width = $total_width / $num_cols
        )
    };
    ($num_cols: expr, $total_width: expr, $e0: expr, $e1: expr, $e2: expr) => {
        format!(
            "{:<width$}{:<width$}{:<width$}",
            $e0,
            $e1,
            $e2,
            width = $total_width / $num_cols
        )
    };
}
