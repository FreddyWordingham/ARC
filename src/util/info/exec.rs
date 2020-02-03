//! Executable information.

use std::{env::args, path::Path};

/// Determine the name of the executable.
#[inline]
#[must_use]
pub fn name() -> String {
    let args: Vec<String> = args().collect();
    Path::new(
        &args
            .get(0)
            .expect("Unable to retrieve the command line arguments."),
    )
    .file_name()
    .expect("Unable to determine program name.")
    .to_str()
    .expect("Unable to convert program name into str.")
    .to_string()
}
