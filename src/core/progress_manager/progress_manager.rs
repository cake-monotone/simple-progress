use std::io::Write;

use crate::components::Component;

pub struct ProgressManager<T: Component>
{
    component: T,
    processed: u32,
    total: u32
}

impl <T: Component> ProgressManager<T>
{
    pub fn new(component: T, total: u32) -> Self {
        ProgressManager {
            component: component,
            processed: 0,
            total: total
        }
    }

    pub fn start(&self) {
        print!("\x1b[?25l");

        self._draw_component();
    }

    pub fn update(&mut self, processed: u32, total: u32) {
        self.processed = processed;
        self.total = total;

        self._cursor_up();
        self._draw_component();
    }

    pub fn process(&mut self) {
        self.processed += 1;

        self._cursor_up();
        self._draw_component();
    }

    pub fn _cursor_up(&self) {
        print!("\x1b[{}A\x1b[1000D", self.component.height());
    }

    pub fn _draw_component(&self) {
        print!("{}\n", self.component.draw(self.processed, self.total));
        std::io::stdout().flush().unwrap();
    }

    pub fn end(&self) {
        print!("\x1b[?25h");
    }
}
