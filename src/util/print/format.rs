//! Formatting functions.

use std::fmt::{Display, Write};

/// Format an array of data into a string of evenly spaced columns.
#[inline]
#[must_use]
pub fn cols<I>(data: I, num_cols: usize, col_width: usize) -> String
where
    I: IntoIterator,
    I::Item: Display,
{
    debug_assert!(num_cols > 0);
    debug_assert!(col_width > 0);

    let mut output = String::new();

    let mut len = 0;
    for (i, item) in data.into_iter().enumerate() {
        write!(output, "{:col_width$}", item, col_width = col_width)
            .expect("Unable to write to string.");
        if i % num_cols == num_cols - 1 {
            writeln!(output).expect("Unable to write to string.");
        }
        len += 1;
    }

    if len % num_cols == 0 {
        output.pop();
    }

    output
}
