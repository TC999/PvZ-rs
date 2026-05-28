// 对应 C++ 中的 ScrollbuttonWidget.h / ScrollbuttonWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct ScrollbuttonWidget {
    pub widget: Widget,
    pub is_up: bool,
    pub is_down: bool,
}

impl ScrollbuttonWidget {
    pub fn new(is_up: bool) -> Self {
        log::debug!("ScrollbuttonWidget::new: 创建滚动按钮，向上={}", is_up);
        Self {
            widget: Widget::new(0, 0, 16, 16),
            is_up,
            is_down: false,
        }
    }
}

impl WidgetLike for ScrollbuttonWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {}
    fn mouse_down(&mut self, _x: i32, _y: i32, _c: i32) { self.is_down = true; }
    fn mouse_up(&mut self, _x: i32, _y: i32, _c: i32) { self.is_down = false; }
}
