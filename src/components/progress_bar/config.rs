#[derive(Debug, Copy, Clone)]
pub(crate) struct ProgressBarConfig {
    pub filled: char,
    pub empty: char,
    pub arrow: Option<char>,
    pub left_border: Option<char>,
    pub right_border: Option<char>,
    pub error: char,
    pub bar_len: u16,
}
