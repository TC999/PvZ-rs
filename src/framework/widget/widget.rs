// 对应 C++ 中的 Widget.h / Widget.cpp
// 基础 UI 控件

use crate::framework::graphics::color::Color;
use crate::framework::graphics::graphics::Graphics;
use crate::framework::misc::rect::TRect;

pub struct Widget {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub enabled: bool,
    pub has_focus: bool,
    pub has_transparencies: bool,
    pub m_do_not_resize: bool,
    pub m_parent: Option<usize>,
    pub m_id: usize,
    pub m_disabled_count: i32,
    pub m_colors: Vec<Color>,
}

impl Widget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x, y, width, height,
            visible: true,
            enabled: true,
            has_focus: false,
            has_transparencies: false,
            m_do_not_resize: false,
            m_parent: None,
            m_id: 0,
            m_disabled_count: 0,
            m_colors: Vec::new(),
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.x = x;
        self.y = y;
        self.width = w;
        self.height = h;
    }

    pub fn get_rect(&self) -> TRect {
        TRect::new(self.x, self.y, self.width, self.height)
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        self.get_rect().contains(px, py)
    }

    pub fn update(&mut self) {
        // 基类更新逻辑
    }

    pub fn draw(&self, _g: &Graphics) {
        // 基类绘制逻辑
    }

    pub fn mouse_down(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    pub fn mouse_up(&mut self, _x: i32, _y: i32, _click_count: i32) {}
    pub fn mouse_move(&mut self, _x: i32, _y: i32) {}
    pub fn mouse_enter(&mut self) {}
    pub fn mouse_leave(&mut self) {}
    pub fn key_down(&mut self, _key: i32) {}
    pub fn key_up(&mut self, _key: i32) {}
    pub fn char_input(&mut self, _ch: char) {}
    pub fn got_focus(&mut self) {}
    pub fn lost_focus(&mut self) {}
}
