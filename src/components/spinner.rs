use super::Component;

pub struct SpinnerComponent {
    pub animations: Vec<String>,
}

impl SpinnerComponent {
    pub fn from_str(animations: &str) -> SpinnerComponent {
        SpinnerComponent {
            animations: animations.chars().map(|x| x.to_string()).collect(),
        }
    }
}

impl Component for SpinnerComponent {
    fn draw(&self, progress: u32, _: u32) -> String {
        if self.animations.is_empty() {
            return String::new();
        }

        self.animations
            .get(progress as usize % self.animations.len())
            .unwrap()
            .clone()
    }
}

#[cfg(test)]
mod test {
    use crate::components::Component;

    use super::SpinnerComponent;

    #[test]
    fn display_test() {
        let spinner = SpinnerComponent::from_str("1234");

        assert_eq!(spinner.draw(0, 4), "1");
        assert_eq!(spinner.draw(1, 4), "2");
        assert_eq!(spinner.draw(2, 4), "3");
        assert_eq!(spinner.draw(3, 4), "4");
        assert_eq!(spinner.draw(4, 4), "1");
    }
}
