use unicode_width::UnicodeWidthChar;

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub filled: char,
    pub empty: char,
    pub arrow: Option<char>,
    pub left_border: Option<char>,
    pub right_border: Option<char>,
    pub error: char,
    pub bar_width: u16,
}

#[derive(Debug)]
pub struct ErrMsg(String);

impl Config {
    pub fn check_invalid_chars(&self) -> Result<(), ErrMsg> {
        let target_chars: Vec<char> = vec![
            Some(self.filled),
            Some(self.empty),
            self.arrow,
            self.left_border,
            self.right_border,
            Some(self.error)
        ]
            .into_iter()
            .filter_map(|x| x)
            .collect();
        
        let invalid_chars: Vec<char> = target_chars
            .into_iter()
            .filter(
                |&x| {
                    UnicodeWidthChar::width(x)
                        .and_then(
                            |width| {
                                if 1 <= width && width <= 2 {
                                    return Some(width)
                                }
                                else {
                                    return None
                                }
                            }
                        )
                        .is_none()
                }
            )
            .collect();

        if invalid_chars.is_empty() {
            return Ok(());
        }
        else {
            let joined_invalid_chars: String = invalid_chars
                .into_iter()
                .map(|ch| format!("'{}' ({:#x})", ch, ch as u32))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(
                ErrMsg(
                    format!(
                        concat!(
                            "Invalid Char Error.\n",
                            "following characters is invalid for progress bar\n",
                            "{}"
                        ),
                        joined_invalid_chars
                    )
                )
            );
        }
    }
}