use crate::components::Component;

#[derive(Debug)]
pub struct Label {
    pub text: String
}


impl Component for Label {
    fn height(&self) -> u16 {
        1u16
    }

    fn draw(&self, _: u32, _: u32) -> String {
        self.text.clone()
    }
}

