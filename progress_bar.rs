pub struct ProgressBar {
    bar: indicatif::ProgressBar,
    current_step: u64,
}

impl ProgressBar {
    pub fn new(total_steps: u64) -> ProgressBar {
        let bar = indicatif::ProgressBar::new(total_steps);
        let current_step = 0;

        ProgressBar {
            bar,
            current_step,
        }
    }
    pub fn set_step(&mut self, step: u64) {
        self.bar.set_position(step);
        self.current_step = step;
    }
}
