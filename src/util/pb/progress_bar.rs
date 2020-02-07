//! Progress-Bar implementation.

/// Progress-bar structure.
pub struct ProgressBar {
    /// Graphics.
    pb: indicatif::ProgressBar,
}

impl ProgressBar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &str, total: u64) -> Self {
        let pb = indicatif::ProgressBar::new(total);
        pb.set_message(msg);

        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .progress_chars("\\/")
        );

        Self { pb }
    }

    /// Tick the bar forward once.
    #[inline]
    pub fn tick(&mut self) {
        self.pb.inc(1);
    }

    /// Finish with a message.
    #[inline]
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
