mod progress_bar;
mod spinner;
mod timer;

pub trait Component {
    fn draw(&self, progress: u32, total: u32) -> String;
}

pub use progress_bar::{ProgressBar, ProgressBarBuilder};
pub use spinner::SpinnerComponent;
