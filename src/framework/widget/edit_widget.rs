// 对应 C++ 中的 EditWidget.h / EditWidget.cpp

use super::widget::Widget;
use super::widget_manager::WidgetLike;
use crate::framework::graphics::graphics::Graphics;

pub struct EditWidget {
    pub widget: Widget,
    pub text: String,
    pub max_chars: usize,
    pub cursor_pos: usize,
    pub has_focus: bool,
    pub password_mode: bool,
}

impl EditWidget {
    pub fn new(max_chars: usize) -> Self {
        Self {
            widget: Widget::new(0, 0, 200, 30),
            text: String::new(),
            max_chars,
            cursor_pos: 0,
            has_focus: false,
            password_mode: false,
        }
    }

    pub fn get_text(&self) -> &str { &self.text }
    pub fn set_text(&mut self, text: &str) { self.text = text.to_string(); }
}

impl WidgetLike for EditWidget {
    fn as_widget(&self) -> &Widget { &self.widget }
    fn as_widget_mut(&mut self) -> &mut Widget { &mut self.widget }
    fn draw(&self, _g: &Graphics) {
        if !self.widget.visible { return; }
        // TODO: 绘制文本框
    }
}
