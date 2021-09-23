use super::ProgressBarConfig;

use crate::components::Component;
use std::convert::TryFrom;
use unicode_width::UnicodeWidthChar;

pub struct ProgressBar {
    config: ProgressBarConfig,
}

pub struct ProgressBarBuilder {
    config: ProgressBarConfig,
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBarBuilder::new().build()
    }

    fn body_char_width(&self) -> u16 {
        u16::try_from(UnicodeWidthChar::width(self.config.filled).unwrap()).unwrap()
    }
}

impl Component for ProgressBar {
    fn draw(&self, progress: u32, total: u32) -> String {
        let config = self.config;

        // processed_length = (progress * bar_length) / total
        // note that 0 <= processed_length <= bar_length(u16)
        let processed_len = (progress as u64 * config.bar_len as u64)
            .checked_div(total as u64)
            .and_then(|x| {
                if x > (config.bar_len as u64) {
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
                    if processed_len != config.bar_len && processed_len >= body_char_width {
                        s.pop();
                        s.push(arrow);
                    }
                }

                // draw status: [=>*
                for _ in (0..(config.bar_len - drawed_len)).step_by(body_char_width as usize) {
                    s.push(config.empty);
                }
            }
            None => {
                // draw status: [???
                for _ in (0..config.bar_len).step_by(body_char_width as usize) {
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
    const DEFAULT_EMPTY_CHAR: char = ' ';
    const DEFAULT_ERROR_CHAR: char = '!';

    const DEFAULT_FULL_WIDTH_EMPTY_CHAR: char = '　';
    const DEFAULT_FULL_WIDTH_ERROR_CHAR: char = '！';

    pub fn new() -> Self {
        ProgressBarBuilder {
            config: ProgressBarConfig {
                filled: '=',
                empty: ' ',
                arrow: Some('>'),
                left_border: Some('['),
                right_border: Some(']'),
                error: '?',
                bar_len: 16u16,
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

    pub fn set_border(&mut self, left: char, right: char) -> &mut Self {
        self.config.left_border = Some(left);
        self.config.right_border = Some(right);
        self
    }

    // TODO: builder complete

    pub fn build(&self) -> ProgressBar {
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
                config: ProgressBarConfig {
                    filled: '=',
                    empty: ' ',
                    arrow: Some('>'),
                    left_border: Some('['),
                    right_border: Some(']'),
                    error: '?',
                    bar_len: 3,
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
                config: ProgressBarConfig {
                    filled: '～',
                    empty: '　',
                    arrow: None,
                    left_border: None,
                    right_border: None,
                    error: '！',
                    bar_len: 6,
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
