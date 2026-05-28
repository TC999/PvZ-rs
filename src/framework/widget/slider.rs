// 对应 C++ 中的 Slider.h / Slider.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct Slider {
    pub widget: Widget,
    pub value: i32,
    pub min_val: i32,
    pub max_val: i32,
    pub is_dragging: bool,
    pub horizontal: bool,
}

impl Slider {
    pub fn new(horizontal: bool) -> Self {
        Self {
            widget: Widget::new(0, 0, 100, 20),
            value: 0,
            min_val: 0,
            max_val: 100,
            is_dragging: false,
            horizontal,
        }
    }

    pub fn get_value(&self) -> i32 { self.value }
    pub fn set_value(&mut self, val: i32) {
        self.value = val.clamp(self.min_val, self.max_val);
    }
}

impl WidgetLike for Slider {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {}
    fn mouse_down(&mut self, _x: i32, _y: i32, _c: i32) { self.is_dragging = true; }
    fn mouse_up(&mut self, _x: i32, _y: i32, _c: i32) { self.is_dragging = false; }
    fn mouse_move(&mut self, x: i32, _y: i32) {
        if self.is_dragging {
            let ratio = (x - self.widget.x) as f32 / self.widget.width as f32;
            self.value = (self.min_val as f32 + ratio * (self.max_val - self.min_val) as f32) as i32;
            self.value = self.value.clamp(self.min_val, self.max_val);
        }
    }
}
