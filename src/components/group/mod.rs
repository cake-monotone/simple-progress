use crate::components::Component;
use super::{Label, ProgressBar};

pub struct LabeledProgressBar {
    pub bar: ProgressBar,
    pub label: Label,
}

impl Component for LabeledProgressBar {
    fn height(&self) -> u16 {
        1u16
    }

    fn draw(&self, progress: u32, total: u32) -> String {
        format!(
            "{}: {}",
            self.bar.draw(progress, total),
            self.label.draw(progress, total)
        )
    }
}

