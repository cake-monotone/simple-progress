use crate::components::{Component, ProgressBar};

use std::io::Write;

pub struct ProgressManager {
    pub bar: ProgressBar,
}

impl ProgressManager {
    pub fn start(&self) {
        print!("\x1b[?25l");
        print!("\x1b[100D{}", self.bar.draw(0u32, 1u32));
    }

    pub fn update(&self, progress: u32, total: u32) {
        print!("\x1b[100D{}", self.bar.draw(progress, total));
        std::io::stdout().flush().unwrap();
    }

    pub fn end(&self) {
        print!("\x1b[?25h");
    }
}
