pub trait Component {
    fn draw(&self, progress: u32, total: u32) -> String;

    fn height(&self) -> u16;
}
