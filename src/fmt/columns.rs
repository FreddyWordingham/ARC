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

// /// Format an array of data into a string of evenly spaced columns.
// #[inline]
// #[must_use]
// pub fn cols<I>(data: I, num_cols: usize, col_width: usize) -> String
// where
//     I: IntoIterator,
//     I::Item: Display,
// {
//     debug_assert!(num_cols > 0);
//     debug_assert!(col_width > 0);

//     let mut output = String::new();

//     let mut len = 0;
//     for (i, item) in data.into_iter().enumerate() {
//         write!(output, "{:col_width$}", item, col_width = col_width)
//             .expect("Unable to write to string.");
//         if i % num_cols == num_cols - 1 {
//             writeln!(output).expect("Unable to write to string.");
//         }
//         len += 1;
//     }

//     if len % num_cols == 0 {
//         output.pop();
//     }

//     output
// }
