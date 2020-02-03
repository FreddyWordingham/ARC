//! Parallel progress-Bar implementation.

/// Parallel progress-bar structure.
pub struct ParProgressBar {
    /// Graphics.
    pb: indicatif::ProgressBar,
    /// Current value.
    count: u64,
}

impl ParProgressBar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &str, total: u64) -> Self {
        let pb = indicatif::ProgressBar::new(total);
        pb.set_message(msg);

        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .progress_chars("\\/")
        );

        Self { pb, count: 0 }
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
        if self.count >= self.pb.position() {
            None
        } else {
            let remaining = self.pb.position() - self.count;
            let alloc = if remaining < size { remaining } else { size };

            let block = Some((self.count, self.count + alloc));

            self.count += alloc;

            block
        }
    }
}
