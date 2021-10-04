use super::Config;
use crate::components::Component;

use std::convert::TryFrom;
use unicode_width::UnicodeWidthChar;

pub struct ProgressBar {
    config: Config,
}

pub struct ProgressBarBuilder {
    config: Config,
}

#[derive(Debug)]
pub struct ProgressBarBuildError;

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBarBuilder::new().build()
    }

    fn body_char_width(&self) -> u16 {
        u16::try_from(UnicodeWidthChar::width(self.config.filled).unwrap()).unwrap()
    }
}

impl Component for ProgressBar {
    fn height(&self) -> u16 {
        1u16
    }

    fn draw(&self, progress: u32, total: u32) -> String {
        let config = self.config;

        // processed_length = (progress * bar_length) / total
        // note that 0 <= processed_length <= bar_length(u16)
        let processed_len = (progress as u64 * config.bar_width as u64)
            .checked_div(total as u64)
            .and_then(|x| {
                if x > (config.bar_width as u64) {
                    None
                } else {
                    Some(u16::try_from(x).unwrap())
                }
            });

        let mut s = String::new();

        // draw status: [
        if let Some(left_border) = config.left_border {
            s.push(left_border);
        }

        let body_char_width = self.body_char_width();
        match processed_len {
            Some(processed_len) => {
                let mut drawed_len = 0;

                // draw status: [=
                for _ in (0..processed_len).step_by(body_char_width as usize) {
                    s.push(config.filled);
                    drawed_len += body_char_width;
                }

                // draw status: [=>
                if let Some(arrow) = config.arrow {
                    if processed_len != config.bar_width && processed_len >= body_char_width {
                        s.pop();
                        s.push(arrow);
                    }
                }

                // draw status: [=>*
                for _ in (0..(config.bar_width - drawed_len)).step_by(body_char_width as usize) {
                    s.push(config.empty);
                }
            }
            None => {
                // draw status: [???
                for _ in (0..config.bar_width).step_by(body_char_width as usize) {
                    s.push(config.error);
                }
            }
        }

        // draw status: [=>*]
        if let Some(ch) = config.right_border {
            s.push(ch);
        }

        s
    }
}

impl ProgressBarBuilder {
    pub fn new() -> Self {
        ProgressBarBuilder {
            config: Config {
                filled: '=',
                empty: ' ',
                arrow: Some('>'),
                left_border: Some('['),
                right_border: Some(']'),
                error: '?',
                bar_width: 16u16,
            },
        }
    }

    pub fn set_filled(&mut self, ch: char) -> &mut Self {
        self.config.filled = ch;
        self
    }

    pub fn set_empty(&mut self, ch: char) -> &mut Self {
        self.config.filled = ch;
        self
    }

    pub fn set_arrow(&mut self, ch: Option<char>) -> &mut Self {
        self.config.arrow = ch;
        self
    }

    pub fn set_border(&mut self, left: Option<char>, right: Option<char>) -> &mut Self {
        self.config.left_border = left;
        self.config.right_border = right;
        self
    }

    pub fn set_bar_width(&mut self, bar_width: u16) -> &mut Self {
        self.config.bar_width = bar_width;
        self
    }

    pub fn build(&self) -> ProgressBar {
        self.config.check_invalid_chars().unwrap();

        ProgressBar {
            config: self.config,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod progress_bar {
        use super::*;

        #[test]
        fn test_new() {
            ProgressBar::new();
        }

        #[test]
        fn test_draw() {
            let progress_bar = ProgressBar {
                config: Config {
                    filled: '=',
                    empty: ' ',
                    arrow: Some('>'),
                    left_border: Some('['),
                    right_border: Some(']'),
                    error: '?',
                    bar_width: 3,
                },
            };

            assert_eq!(progress_bar.draw(0, 3), "[   ]");
            assert_eq!(progress_bar.draw(1, 3), "[>  ]");
            assert_eq!(progress_bar.draw(2, 3), "[=> ]");
            assert_eq!(progress_bar.draw(3, 3), "[===]");

            assert_eq!(progress_bar.draw(4, 3), "[???]");
            assert_eq!(progress_bar.draw(3, 0), "[???]");

            assert_eq!(progress_bar.draw(1, 3), progress_bar.draw(3, 9));

            let full_width_char_progress_bar = ProgressBar {
                config: Config {
                    filled: '～',
                    empty: '　',
                    arrow: None,
                    left_border: None,
                    right_border: None,
                    error: '！',
                    bar_width: 6,
                },
            };

            assert_eq!(full_width_char_progress_bar.draw(0, 5), "　　　");
            assert_eq!(full_width_char_progress_bar.draw(1, 5), "～　　");
            assert_eq!(full_width_char_progress_bar.draw(2, 5), "～　　");
            assert_eq!(full_width_char_progress_bar.draw(3, 5), "～～　");
            assert_eq!(full_width_char_progress_bar.draw(4, 5), "～～　");
            assert_eq!(full_width_char_progress_bar.draw(5, 5), "～～～");

            assert_eq!(full_width_char_progress_bar.draw(6, 5), "！！！");
            assert_eq!(full_width_char_progress_bar.draw(6, 0), "！！！");
        }
    }
}
