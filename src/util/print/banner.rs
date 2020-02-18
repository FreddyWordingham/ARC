//! Banner functions.

use colored::Colorize;
use terminal_size::terminal_size;

/// Fallback terminal width if it cannot be determined at runtime.
const FALLBACK_TERM_WIDTH: usize = 80;

/// Determine the terminal width.
#[inline]
#[must_use]
fn term_width() -> usize {
    let ts = terminal_size();

    if let Some(ts) = ts {
        (ts.0).0 as usize
    } else {
        FALLBACK_TERM_WIDTH
    }
}

/// Print a main title bar.
#[inline]
pub fn title(title: &str) {
    let title = title.to_uppercase();

    let term_width = term_width();

    let (left_bar, right_bar) = if term_width < ((title.len() * 2) + 11) {
        (4, 4)
    } else {
        let left_bar = (term_width - (title.len() * 2) - 3) / 2;
        (left_bar, term_width - (title.len() * 2) - 3 - left_bar)
    };

    print!("{} ", "\u{2588}".repeat(left_bar));

    for (pos, ch) in title.chars().enumerate() {
        match pos % 6 {
            0 => print!(" {}", format!("{}", ch).bright_red().bold()),
            1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
            2 => print!(" {}", format!("{}", ch).bright_green().bold()),
            3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
            4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
            5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
            _ => unreachable!(),
        }
    }

    println!("  {}", "\u{2588}".repeat(right_bar));
}

/// Print a section bar.
#[inline]
pub fn section(title: &str) {
    let term_width = term_width();

    print!("\n====");

    unsafe {
        /// Index of the section.
        static mut SECTION: i32 = 0;
        match SECTION % 6 {
            0 => print!(" {}", title.bright_red().bold()),
            1 => print!(" {}", title.bright_yellow().bold()),
            2 => print!(" {}", title.bright_green().bold()),
            3 => print!(" {}", title.bright_cyan().bold()),
            4 => print!(" {}", title.bright_blue().bold()),
            5 => print!(" {}", title.bright_magenta().bold()),
            _ => unreachable!(),
        }
        SECTION += 1;
    }

    let mut cur_len = 5 + title.len();

    if cur_len >= term_width {
        println!();
        return;
    }

    print!(" ");
    cur_len += 1;
    while cur_len < term_width {
        print!("=");
        cur_len += 1;
    }

    println!();
}
