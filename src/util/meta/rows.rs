//! Rows macro.

/// Report a row of values in equally spaced columns.
#[macro_export]
macro_rules! rows {
    ($zeroth: expr) => {
        log::info!("{:<31}", $zeroth);
    };

    ($zeroth: expr, $one: expr) => {
        log::info!("{:<31}, {:<31}", $zeroth, $one);
    };
}
