use indicatif::ProgressBar;
use std::time::Duration;

pub struct Spinner(ProgressBar);

impl Spinner {
    pub fn start() -> Self {
        let tick_rate = Duration::from_millis(500);
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(tick_rate);

        Spinner(spinner)
    }

    pub fn set_msg(&self, msg: &str) {
        self.0.set_message(msg.to_string());
    }

    pub fn finish(&self) {
        self.0.finish_and_clear();
    }
}
