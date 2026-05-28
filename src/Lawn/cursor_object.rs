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
        log::debug!("CursorObject::new: 创建光标对象");
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
        log::trace!("CursorObject::update: 更新光标对象，位置 ({}, {})", self.x, self.y);
        self.anim_counter += 1;
    }

    pub fn draw(&self) {
        log::trace!("CursorObject::draw: 绘制光标对象");
        // TODO: 绘制光标
    }
}
