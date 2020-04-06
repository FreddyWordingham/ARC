//! Columns macro.

use crate::fmt::term_width;
use std::fmt::Display;

/// Report a list of values in evenly spaced columns.
#[macro_export]
macro_rules! columns {
    ($col_width: expr, $($val: expr),*) => {
        {
            use crate::fmt::term_width;

            let num_cols = (term_width() / $col_width).max(1);
            let mut index = 0;
            $(
                {
                    print!("{:<cw$}", $val, cw = $col_width);
                    if index % num_cols == num_cols - 1 {
                        println!();
                    }
                    index += 1;
                }
            )*

            if index % num_cols != 0 {
                println!();
            }
        }
    };
}

/// Print a given iterator into equally spaced columns.
#[inline]
pub fn columns<I>(col_width: usize, values: I)
where
    I: IntoIterator,
    I::Item: Display,
{
    let num_cols = (term_width() / col_width).max(1);

    let mut index = 0;
    for val in values {
        print!("{:<cw$}", val, cw = col_width);
        if index % num_cols == num_cols - 1 {
            println!();
        }
        index += 1;
    }

    if index % num_cols != 0 {
        println!();
    }
}
