//! Parallel progress-Bar implementation.

/// Parallel progress-bar structure.
pub struct ParProgressBar {
    /// Graphics.
    pb: indicatif::ProgressBar,
    /// Current value.
    count: u64,
    /// Total target value.
    total: u64,
}

impl ParProgressBar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &str, total: u64) -> Self {
        debug_assert!(total > 0);

        let pb = indicatif::ProgressBar::new(total);

        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .progress_chars("\\/")
        );
        pb.set_message(msg);

        Self {
            pb,
            count: 0,
            total,
        }
    }

    /// Tick the bar forward once.
    #[inline]
    pub fn tick(&mut self) {
        self.pb.inc(1);
    }

    /// Request a block of values to work on.
    /// Return the requested block if available.
    /// If there is not enough, return the remaining block.
    /// If there are none at all, return None.
    #[inline]
    #[must_use]
    pub fn block(&mut self, size: u64) -> Option<(u64, u64)> {
        debug_assert!(size > 0);

        if self.count >= self.total {
            None
        } else {
            let remaining = self.total - self.count;
            let alloc = if remaining < size { remaining } else { size };

            let block = Some((self.count, self.count + alloc));

            self.count += alloc;
            self.pb.inc(alloc);

            block
        }
    }

    /// Finish with a message.
    #[inline]
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
