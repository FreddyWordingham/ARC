//! Formatting module.

mod banner;
mod cols;
mod columns;
mod values;

pub use self::{banner::*, cols::*, columns::*, values::*};

use terminal_size::terminal_size;

/// Fallback terminal width if it cannot be determined at runtime.
const FALLBACK_TERM_WIDTH: usize = 80;

/// Determine the terminal width.
#[inline]
#[must_use]
pub fn term_width() -> usize {
    let ts = terminal_size();

    if let Some(ts) = ts {
        (ts.0).0 as usize
    } else {
        FALLBACK_TERM_WIDTH
    }
}
