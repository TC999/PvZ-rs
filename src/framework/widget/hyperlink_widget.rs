// 对应 C++ 中的 HyperlinkWidget.h / HyperlinkWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::color::Color;
use crate::framework::graphics::graphics::Graphics;

pub struct HyperlinkWidget {
    pub widget: Widget,
    pub url: String,
    pub label: String,
    pub color: Color,
    pub is_over: bool,
}

impl HyperlinkWidget {
    pub fn new(url: &str, label: &str) -> Self {
        Self {
            widget: Widget::new(0, 0, 200, 20),
            url: url.to_string(),
            label: label.to_string(),
            color: Color::new(0, 0, 255, 255),
            is_over: false,
        }
    }
}

impl WidgetLike for HyperlinkWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
    }
    fn mouse_enter(&mut self) { self.is_over = true; }
    fn mouse_leave(&mut self) { self.is_over = false; }
    fn mouse_up(&mut self, x: i32, y: i32, _c: i32) {
        if self.widget.contains(x, y) {
            // TODO: 打开 URL
        }
    }
}
