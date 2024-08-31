use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct VimGame {
    cursor_x: usize,
    cursor_y: usize,
    max_x: usize,
    max_y: usize,
}

#[wasm_bindgen]
impl VimGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&"VimGame initialized".into());
        Self {
            cursor_x: 0,
            cursor_y: 0,
            max_x: 10,
            max_y: 10,
        }
    }

    pub fn process_keypress(&mut self, key: &str) -> String {
        match key {
            "h" => self.move_cursor(-1, 0),
            "j" => self.move_cursor(0, 1),
            "k" => self.move_cursor(0, -1),
            "l" => self.move_cursor(1, 0),
            _ => console::log_1(&"Unhandled key".into()),
        }
        self.log_cursor_position();
        self.get_cursor_position()
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        let new_x = (self.cursor_x as i32 + dx).max(0).min(self.max_x as i32) as usize;
        let new_y = (self.cursor_y as i32 + dy).max(0).min(self.max_y as i32) as usize;
        self.cursor_x = new_x;
        self.cursor_y = new_y;
    }

    fn log_cursor_position(&self) {
        console::log_1(&format!("Cursor position: ({}, {})", self.cursor_x, self.cursor_y).into());
    }

    pub fn get_cursor_position(&self) -> String {
        format!("({}, {})", self.cursor_x, self.cursor_y)
    }
}