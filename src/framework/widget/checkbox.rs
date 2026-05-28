// 对应 C++ 中的 Checkbox.h / Checkbox.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct Checkbox {
    pub widget: Widget,
    pub label: String,
    pub checked: bool,
    pub is_down: bool,
}

impl Checkbox {
    pub fn new(label: &str) -> Self {
        Self {
            widget: Widget::new(0, 0, 200, 30),
            label: label.to_string(),
            checked: false,
            is_down: false,
        }
    }

    pub fn is_checked(&self) -> bool { self.checked }
    pub fn set_checked(&mut self, checked: bool) { self.checked = checked; }
}

impl WidgetLike for Checkbox {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制复选框
    }
    fn mouse_down(&mut self, x: i32, y: i32, _c: i32) {
        if self.widget.contains(x, y) { self.is_down = true; }
    }
    fn mouse_up(&mut self, x: i32, y: i32, _c: i32) {
        if self.is_down && self.widget.contains(x, y) {
            self.checked = !self.checked;
        }
        self.is_down = false;
    }
}
