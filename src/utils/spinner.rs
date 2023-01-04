use indicatif::ProgressBar;
use std::time::Duration;

pub(crate) struct Spinner(ProgressBar);

impl Spinner {
    pub(crate) fn start() -> Self {
        let tick_rate = Duration::from_millis(500);
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(tick_rate);

        Spinner(spinner)
    }

    pub(crate) fn set_msg(&self, msg: &str) {
        self.0.set_message(msg.to_string());
    }

    pub(crate) fn finish(&self) {
        self.0.finish_and_clear();
    }
}
