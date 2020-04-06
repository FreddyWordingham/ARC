//! Formatting functions.

use std::fmt::{Display, Write};

/// Format an array of data into a string of evenly spaced columns.
#[inline]
#[must_use]
pub fn cols<T: Display>(data: &[T], num_cols: usize, col_width: usize) -> String {
    debug_assert!(num_cols > 0);
    debug_assert!(col_width > 0);

    let mut output = String::new();

    for (i, item) in data.iter().enumerate() {
        write!(output, "{:col_width$}", item, col_width = col_width)
            .expect("Unable to write to string.");
        if i % num_cols == num_cols - 1 {
            writeln!(output).expect("Unable to write to string.");
        }
    }

    if data.len() % num_cols == 0 {
        output.pop();
    }

    output
}
