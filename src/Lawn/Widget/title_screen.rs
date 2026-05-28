// 对应 C++ 中的 TitleScreen.h / TitleScreen.cpp
use crate::framework::graphics::graphics::Graphics;

pub struct TitleScreen {
    pub active: bool,
    pub state: i32,
    pub counter: i32,
    pub showing: bool,
}

impl TitleScreen {
    pub fn new() -> Self {
        Self { active: false, state: 0, counter: 0, showing: false }
    }
    pub fn update(&mut self) { if self.active { self.counter += 1; } }
    pub fn draw(&self, _g: &Graphics) {}
    pub fn start(&mut self) { self.active = true; self.state = 0; }
}
