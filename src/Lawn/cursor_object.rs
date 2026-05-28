// 对应 C++ 中的 CursorObject.h / CursorObject.cpp

use crate::const_enums::*;

pub struct CursorObject {
    pub cursor_type: CursorType,
    pub x: i32,
    pub y: i32,
    pub seed_type: SeedType,
    pub visible: bool,
    pub anim_counter: i32,
}

impl CursorObject {
    pub fn new() -> Self {
        Self {
            cursor_type: CursorType::Normal,
            x: 0,
            y: 0,
            seed_type: SeedType::None,
            visible: true,
            anim_counter: 0,
        }
    }

    pub fn update(&mut self) {
        self.anim_counter += 1;
    }

    pub fn draw(&self) {
        // TODO: 绘制光标
    }
}
