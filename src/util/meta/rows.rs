//! Rows macro.

/// Report a row of values in equally spaced columns.
#[macro_export]
macro_rules! rows {
    ($zeroth: expr) => {
        log::info!("{:<32}", $zeroth);
    };

    ($zeroth: expr, $one: expr) => {
        log::info!("{:<32}{:^16}", $zeroth, $one);
    };

    ($zeroth: expr, $one: expr, $two: expr) => {
        log::info!("{:<32}{:^16}{:^16}", $zeroth, $one, $two);
    };

    ($zeroth: expr, $one: expr, $two: expr, $three: expr) => {
        log::info!("{:<32}{:^16}{:^16}{:^16}", $zeroth, $one, $two, $three,);
    };

    ($zeroth: expr, $one: expr, $two: expr, $three: expr, $four: expr) => {
        log::info!(
            "{:<32}{:^16}{:^16}{:^16}{:^16}",
            $zeroth,
            $one,
            $two,
            $three,
            $four
        );
    };
}
